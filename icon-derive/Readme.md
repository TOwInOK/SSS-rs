# tabler-icon-definer

A Rust procedural macro crate for embedding Tabler SVG icons into your binary at compile time.

## Features

- Downloads SVG icons during compilation
- Caches icons locally for 30 days in `target/icon_cache`
- Fetches Tabler icons from the upstream repository
- Generates a type-safe `Tabler` enum for icon access
- Supports custom display names through `#[name = "..."]`
- Allows extra enum attributes such as `#[derive(...)]`
- Optionally generates [`Tabler::to_leptos()`](icon-derive/src/lib.rs) behind the [`leptos`](icon-derive/Cargo.toml) feature

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tabler-icon-definer = "2"
```

If you need [`Tabler::to_leptos()`](icon-derive/src/lib.rs), the consuming crate should own the `leptos` feature and map it both to its local `leptos` dependency and to [`tabler-icon-definer/leptos`](icon-derive/Cargo.toml):

```toml
[dependencies]
tabler-icon-definer = "2"
leptos = { version = "0.8", optional = true }

[features]
leptos = ["dep:leptos", "tabler-icon-definer/leptos"]
```

## Usage

### Basic usage

```rust
use tabler_icon_definer::tabler_icon;

tabler_icon!(
    brand_github[outline, filled],
    user[outline]
);

// Generates:
// - Tabler::OUTLINE_BRAND_GITHUB
// - Tabler::FILLED_BRAND_GITHUB
// - Tabler::OUTLINE_USER
```

### Custom names

```rust
tabler_icon!(
    #[name = "github"]
    brand_github[outline, filled]
);

// Generates:
// - Tabler::OUTLINE_GITHUB
// - Tabler::FILLED_GITHUB
```

### Additional derives and enum attributes

```rust
tabler_icon!(
    #[derive(serde::Serialize)]
    #[name = "github"]
    brand_github[outline, filled]
);
```

Macro-level attributes are applied to the generated enum. A top-level `#[name = "..."]` still renames the first icon, preserving the existing macro behavior.

### Using generated icons

```rust
let icon = Tabler::OUTLINE_GITHUB;

// Get SVG content
let svg = icon.as_str();

// Get icon name without style prefix
let name = icon.as_str_merget();

// Get full icon name with style prefix
let full_name = icon.to_string();

// Parse from string
let parsed: Tabler = full_name.parse().unwrap();

// Get all available icons
let all_icons = Tabler::all_icons();
```

### Leptos

- add `leptos` as a dependency in the consuming crate
- gate the generated [`Tabler::to_leptos()`](icon-derive/src/lib.rs) method behind a feature in the consuming crate
- map that feature to [`tabler-icon-definer/leptos`](icon-derive/Cargo.toml)

```ignore
let component = Tabler::OUTLINE_GITHUB.to_leptos();
```

## Icon naming

To find the correct icon names, visit [Tabler Icons](https://tabler.io/icons).

The macro keeps the current naming rules:

- the first declared icon becomes [`Default::default()`](https://doc.rust-lang.org/std/default/trait.Default.html)
- variant names are generated as `STYLE_NAME`
- [`Tabler::as_str_merget()`](icon-derive/src/lib.rs) returns the display name without the style prefix
- [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) and [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) use lowercase `style_name`

## Internal layout

The crate is now split into focused modules inside [`icon-derive/src`](icon-derive/src):

- [`lib.rs`](icon-derive/src/lib.rs) — proc-macro entrypoint and crate-level docs
- [`input.rs`](icon-derive/src/input.rs) — parsing the macro input and normalization of top-level `#[name = "..."]`
- [`naming.rs`](icon-derive/src/naming.rs) — variant, display, and download naming helpers
- [`cache.rs`](icon-derive/src/cache.rs) — cache lookup, freshness checks, downloading, and SVG normalization
- [`codegen.rs`](icon-derive/src/codegen.rs) — assembly of the generated `Tabler` enum and impl blocks

Integration tests are also grouped by responsibility inside [`icon-derive/tests`](icon-derive/tests):

- [`common/mod.rs`](icon-derive/tests/common/mod.rs) — shared macro invocation used by the test suite
- [`enum_api.rs`](icon-derive/tests/enum_api.rs) — enum shape, default variant, and ordering checks
- [`string_api.rs`](icon-derive/tests/string_api.rs) — `Display`/`FromStr` behavior
- [`svg_content.rs`](icon-derive/tests/svg_content.rs) — merged names and embedded SVG checks

## Caching

Icons are cached in `target/icon_cache` for 30 days to avoid unnecessary downloads during repeated builds and test runs.

## Notes

- The first icon in the list becomes the default variant
- Icons are downloaded during macro expansion
- Supports both outline and filled styles for Tabler icons

## License

MIT
