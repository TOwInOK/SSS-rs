//! Filesystem discovery helpers used by the layout proc-macro.
//!
//! This module resolves macro arguments relative to the invoking crate, enumerates layout
//! directories, normalizes directory names into runtime strings and Rust identifiers, and validates
//! the invariants that code generation relies on.

use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use proc_macro2::Span;
use syn::{Error, Ident};

#[derive(Debug)]
/// Metadata derived from a single discovered layout directory.
pub(crate) struct LayoutDirectory {
    /// Original directory name as it appears on disk.
    pub(crate) original_name: String,
    /// Normalized lowercase name used for `Display` and `FromStr` generation.
    pub(crate) layout_name: String,
    /// Uppercase identifier used as the generated enum variant name.
    pub(crate) layout_ident: Ident,
    /// Canonical path to the directory on disk.
    pub(crate) path: PathBuf,
}

/// Resolves a macro-provided path relative to the crate that invokes the proc-macro.
pub(crate) fn resolve_macro_path(
    path: &Path,
    kind: &str,
) -> Result<PathBuf, Error> {
    // Proc-macros receive paths relative to the consuming crate, so anchor them to that manifest
    // directory instead of the `html-layouts-derive` crate itself.
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let joined_path = PathBuf::from(&cargo_dir).join(path);

    // Canonicalize eagerly so later stages operate on a stable absolute path and error messages
    // point to the fully resolved filesystem location.
    joined_path.canonicalize().map_err(|error| {
        Error::new(
            Span::call_site(),
            format!(
                "Failed to resolve {kind} path '{}': {error}",
                joined_path.display()
            ),
        )
    })
}

/// Verifies that a required file or directory exists before deeper parsing starts.
pub(crate) fn ensure_exists(
    path: &Path,
    kind: &str,
) -> Result<(), Error> {
    // Fail fast with a targeted compile error instead of letting later I/O or parsing steps fail
    // with less helpful diagnostics.
    if path.exists() {
        return Ok(());
    }

    Err(Error::new(
        Span::call_site(),
        format!("Missing {kind}: {}", path.display()),
    ))
}

/// Reads a UTF-8 text file and maps I/O failures into proc-macro diagnostics.
pub(crate) fn read_utf8_file(
    path: &Path,
    kind: &str,
) -> Result<String, Error> {
    // Template and limitation sources are embedded into generated code as Rust literals, so they
    // must be readable as UTF-8 text.
    read_to_string(path).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("Failed to read {kind} '{}': {error}", path.display()),
        )
    })
}

/// Discovers all immediate child directories that represent layouts under the given root.
pub(crate) fn find_layout_directories(
    root: &Path,
    kind: &str,
) -> Result<Vec<LayoutDirectory>, Error> {
    // Each direct child directory is treated as one layout; files at the root are ignored.
    let entries = read_dir(root).map_err(|error| {
        Error::new(
            Span::call_site(),
            format!(
                "Failed to read {kind} directory '{}': {error}",
                root.display()
            ),
        )
    })?;

    let mut directories = Vec::new();
    for entry in entries {
        // Surface entry iteration errors as compile diagnostics tied to the scanned root.
        let entry = entry.map_err(|error| {
            Error::new(
                Span::call_site(),
                format!(
                    "Failed to read an entry from {kind} directory '{}': {error}",
                    root.display()
                ),
            )
        })?;

        // Query the file type before deciding whether this filesystem entry participates in code
        // generation.
        let file_type = entry.file_type().map_err(|error| {
            Error::new(
                Span::call_site(),
                format!(
                    "Failed to read file type for '{}' in {kind} directory '{}': {error}",
                    entry.path().display(),
                    root.display()
                ),
            )
        })?;

        // Only directories map to layouts; any sibling files are intentionally ignored.
        if !file_type.is_dir() {
            continue;
        }

        // Require UTF-8 names because the generated runtime string names and identifiers are based
        // directly on the directory name.
        let original_name = entry
            .file_name()
            .into_string()
            .map_err(|name| invalid_utf8_directory_name_error(&name, root, kind))?;

        // Normalize and validate the layout metadata as soon as the directory is discovered.
        directories.push(build_layout_directory(
            root,
            kind,
            original_name,
            entry.path(),
        )?);
    }

    // Sort by the normalized name so generated code is deterministic regardless of filesystem
    // iteration order.
    directories.sort_by(|left, right| left.layout_name.cmp(&right.layout_name));

    // The generated enums must contain at least one variant, so an empty layout root is invalid.
    if directories.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            format!("No {kind} directories found in {}", root.display()),
        ));
    }

    // Different original names can collapse to the same lowercase name or uppercase identifier;
    // reject those collisions before code generation starts.
    validate_unique_layouts(&directories, kind)?;

    Ok(directories)
}

/// Converts a discovered directory name into the normalized metadata used by code generation.
fn build_layout_directory(
    root: &Path,
    kind: &str,
    original_name: String,
    path: PathBuf,
) -> Result<LayoutDirectory, Error> {
    // Lowercase names are used for runtime parsing and display so lookups become case-insensitive.
    let layout_name = original_name.to_lowercase();

    // Enum variants are generated from an uppercase form of the directory name. This keeps the
    // generated API visually distinct from the lowercase runtime representation.
    let ident_candidate = original_name.to_uppercase();
    let layout_ident = syn::parse_str::<Ident>(&ident_candidate).map_err(|_| {
        Error::new(
            Span::call_site(),
            format!(
                "Invalid {kind} directory name '{}' in {}: '{}' is not a valid Rust identifier",
                original_name,
                root.display(),
                ident_candidate
            ),
        )
    })?;

    // Return both the original and normalized forms because later validation needs access to both.
    Ok(LayoutDirectory {
        original_name,
        layout_name,
        layout_ident,
        path,
    })
}

/// Ensures that normalized layout names and generated identifiers stay unique.
pub(crate) fn validate_unique_layouts(
    directories: &[LayoutDirectory],
    kind: &str,
) -> Result<(), Error> {
    // Track collisions in the lowercase runtime namespace used by `Display` and `FromStr`.
    let mut normalized_names = BTreeMap::new();

    // Track collisions in the generated Rust identifier namespace used by enum variants.
    let mut identifiers = BTreeMap::new();

    for directory in directories {
        if let Some(previous_name) = normalized_names.insert(
            directory.layout_name.as_str(),
            directory.original_name.as_str(),
        ) {
            // Report both original names so the caller can see which directories collided after
            // normalization.
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "Duplicate normalized {kind} name '{}': '{}' and '{}' both map to the same generated layout name",
                    directory.layout_name, previous_name, directory.original_name
                ),
            ));
        }

        let identifier = directory.layout_ident.to_string();
        if let Some(previous_name) =
            identifiers.insert(identifier.clone(), directory.original_name.as_str())
        {
            // Identifier collisions are checked separately because uppercase normalization can
            // merge names even when the lowercase runtime names remain distinct.
            return Err(Error::new(
                Span::call_site(),
                format!(
                    "Duplicate generated Rust identifier '{}' for {kind} directories '{}' and '{}'",
                    identifier, previous_name, directory.original_name
                ),
            ));
        }
    }

    Ok(())
}

/// Builds a compile-time error for layout directories whose names are not valid UTF-8.
fn invalid_utf8_directory_name_error(
    name: &OsString,
    root: &Path,
    kind: &str,
) -> Error {
    // Preserve the raw `OsString` debug representation because it is the most informative output
    // available for non-UTF-8 paths.
    Error::new(
        Span::call_site(),
        format!(
            "Found non-UTF-8 {kind} directory name {:?} in {}",
            name,
            root.display()
        ),
    )
}

#[cfg(test)]
mod tests {
    //! Tests for directory discovery and normalization rules.

    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::find_layout_directories;

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        /// Creates an empty temporary directory with a unique name for a single test case.
        fn new() -> Self {
            // Combine the process id with a high-resolution timestamp so parallel test runs do not
            // try to reuse the same temporary directory.
            let unique_suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "html-layouts-derive-tests-{}-{unique_suffix}",
                std::process::id()
            ));

            // Create the directory eagerly so helper methods can assume it already exists.
            fs::create_dir_all(&path).expect("failed to create temporary directory");

            Self { path }
        }

        /// Returns the filesystem path of the temporary test directory.
        fn path(&self) -> &Path {
            &self.path
        }

        /// Creates a child directory that simulates a discovered layout.
        fn create_dir(
            &self,
            name: &str,
        ) {
            fs::create_dir(self.path.join(name)).expect("failed to create layout directory");
        }
    }

    impl Drop for TestDir {
        /// Removes the temporary directory after the test finishes.
        fn drop(&mut self) {
            // Ignore cleanup failures so they do not hide the original assertion failure.
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    /// Verifies that layout discovery is sorted by normalized name instead of filesystem order.
    fn sorts_layout_directories_deterministically() {
        let test_dir = TestDir::new();
        test_dir.create_dir("umbrella");
        test_dir.create_dir("castle");
        test_dir.create_dir("github");

        // Discovery should return layouts in deterministic alphabetical order.
        let directories = find_layout_directories(test_dir.path(), "template layout")
            .expect("layout discovery failed");
        let names = directories
            .iter()
            .map(|directory| directory.layout_name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(names, vec!["castle", "github", "umbrella"]);
    }

    #[test]
    /// Verifies that names colliding after lowercase normalization are rejected.
    fn rejects_duplicate_names_after_normalization() {
        use super::{validate_unique_layouts, LayoutDirectory};
        use proc_macro2::Span;
        use syn::Ident;

        // Create LayoutDirectory objects manually without creating actual directories.
        // This avoids filesystem case-sensitivity issues on Windows and macOS.
        let directories = vec![
            LayoutDirectory {
                original_name: "github".to_string(),
                layout_name: "github".to_string(),
                layout_ident: Ident::new("GITHUB", Span::call_site()),
                path: PathBuf::from("/fake/path/github"),
            },
            LayoutDirectory {
                original_name: "GitHub".to_string(),
                layout_name: "github".to_string(),
                layout_ident: Ident::new("GITHUB", Span::call_site()),
                path: PathBuf::from("/fake/path/GitHub"),
            },
        ];

        // These names differ in original form but collapse to the same normalized name.
        let error = validate_unique_layouts(&directories, "template layout")
            .expect_err("duplicate normalized names should fail");

        assert!(
            error
                .to_string()
                .contains("Duplicate normalized template layout name 'github'")
        );
    }
}
