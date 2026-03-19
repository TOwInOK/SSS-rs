# theme-generator

`theme-generator` is a `proc-macro` crate for declaratively describing themes and generating a ready-to-use Rust API on top of them.

The [`generate_theme!`](theme-generator/src/lib.rs:12) macro accepts a set of theme definitions written in a compact DSL and expands them into:

- static [`Theme`](sss-lib/theme/src/lib.rs:13) values for each theme;
- a `Themes` enum with all declared variants;
- accessor methods for colors, fonts, and the full theme;
- conversions from `Themes` into `&'static Theme`;
- [`Display`](theme-generator/Readme.md) and [`FromStr`](theme-generator/Readme.md) implementations for string-based theme selection;
- derive implementations for `serde`, `clap`, and, when configured, `utoipa`.

## What the crate generates

For every theme block, the macro generates:

1. `pub static <NAME_IN_UPPERCASE>: Theme` — a static theme definition;
2. `pub enum Themes` — an enum containing all themes;
3. `impl Themes` with helper methods:
   - `font(&self) -> (&'static str, &'static str)`
   - `colors(&self) -> &Colors`
   - `theme(&self) -> &Theme`
   - `all_themes() -> Vec<Themes>`
4. `impl Display for Themes` — the theme name in lowercase form;
5. `impl FromStr for Themes` — case-insensitive string parsing;
6. `impl From<Themes> for &'static Theme` and `impl From<&Themes> for &'static Theme`.

In addition, `Themes` receives the following derives:

- `Debug`
- `Default`
- `Deserialize`
- `Serialize`
- `Clone`
- `clap::ValueEnum`
- `PartialEq`
- `utoipa::ToSchema` via `cfg_attr(feature = "utoipa", ...)` when that configuration is enabled in the consuming crate

> The first declared theme becomes the default value for `Themes`, because `#[default]` is applied to the first generated variant.

## Input DSL format

The macro expects a strict structure:

```rust
use serde::{Deserialize, Serialize};
use theme::{Colors, Theme};
use theme_generator::generate_theme;

generate_theme! {
    dark {
        colors {
            text: "#ffffff",
            background: "#111111",
            accent: "#5eead4",
            border: "#2a2a2a",
        },
        font {
            name: "Inter",
            link: "https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap"
        }
    }
}
```

Parser expectations:

- the macro does not accept empty input;
- inside each theme, the `colors` block must come first, followed by `font`;
- a comma between `colors` and `font` is required;
- fields inside `colors` must appear in the order `text`, `background`, `accent`, `border`;
- fields inside `font` must appear in the order `name`, `link`;
- the theme name is used:
  - in uppercase for `static` items and enum variants;
  - in lowercase for `Display` and `FromStr`.

For example, a `dark` theme will produce:

- `DARK: Theme`
- `Themes::DARK`
- the string name `"dark"`

## Adding the crate to `Cargo.toml`

The examples below are written for usage inside the current workspace. If you integrate the crate into a different project layout, keep the same dependency and feature setup, but adjust paths or versions as needed.

### Basic setup without feature flags

```toml
[dependencies]
theme-generator = { path = "../theme-generator" }
theme = { path = "../sss-lib/theme" }
serde = { version = "1", features = ["derive"] }
clap = { version = "4.5.23", features = ["derive"] }
```

This is the minimum practical setup for using the macro:

- `theme-generator` provides [`generate_theme!`](theme-generator/src/lib.rs:12);
- the `theme` crate provides [`Theme`](sss-lib/theme/src/lib.rs:13) and `Colors`, which are used in the generated code;
- `serde` is required for `Serialize` and `Deserialize` derives;
- `clap` is required for `clap::ValueEnum`.

### Explicit setup with `default-features = false`

```toml
[dependencies]
theme-generator = { path = "../theme-generator", default-features = false }
theme = { path = "../sss-lib/theme" }
serde = { version = "1", features = ["derive"] }
clap = { version = "4.5.23", features = ["derive"] }
```

This is equivalent to the basic setup because the crate declares `default = []` in [`theme-generator/Cargo.toml`](theme-generator/Cargo.toml).

### Setup with the `utoipa` feature

The recommended approach is to define a feature with the same name in the consuming crate and forward it into the dependency:

```toml
[features]
default = []
utoipa = ["dep:utoipa", "theme-generator/utoipa"]

[dependencies]
theme-generator = { path = "../theme-generator", default-features = false }
theme = { path = "../sss-lib/theme" }
serde = { version = "1", features = ["derive"] }
clap = { version = "4.5.23", features = ["derive"] }
utoipa = { version = "5.3.1", optional = true }
```

After that, you can build the consuming crate with:

```bash
cargo build --features utoipa
```

In this mode, the generated `Themes` enum can also derive `utoipa::ToSchema`.

## Feature flags

### `default = []`

The crate has no optional functionality enabled by default.

Implications:

- the regular dependency form and the form with `default-features = false` behave the same way;
- no integration is enabled automatically.

### `utoipa = []`

This feature is intended for setups where the generated `Themes` enum should participate in OpenAPI schema generation through `utoipa`.

Two details are important:

1. The generated code uses `#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]`.
2. That condition is evaluated in the consuming crate, not inside the `proc-macro` crate itself.

Because of that, a working setup usually requires all of the following:

- defining a `utoipa` feature in your own crate;
- adding `utoipa` as a dependency;
- optionally forwarding the feature into `theme-generator` via `theme-generator/utoipa`.

## Basic usage example

```rust
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use theme::{Colors, Theme};
use theme_generator::generate_theme;

generate_theme! {
    dark {
        colors {
            text: "#ffffff",
            background: "#111111",
            accent: "#5eead4",
            border: "#2a2a2a",
        },
        font {
            name: "Inter",
            link: "https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap"
        }
    }
    light {
        colors {
            text: "#111111",
            background: "#ffffff",
            accent: "#2563eb",
            border: "#d4d4d8",
        },
        font {
            name: "Inter",
            link: "https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap"
        }
    }
}

fn main() {
    let selected = Themes::from_str("DARK").unwrap();

    assert_eq!(selected, Themes::DARK);
    assert_eq!(selected.to_string(), "dark");
    assert_eq!(Themes::default(), Themes::DARK);

    assert_eq!(selected.font().0, "Inter");
    assert_eq!(selected.colors().background, "#111111");

    let theme: &Theme = (&selected).into();
    assert_eq!(theme.colors.accent, "#5eead4");

    let all = Themes::all_themes();
    assert_eq!(all, vec![Themes::DARK, Themes::LIGHT]);

    assert_eq!(DARK.font.0, "Inter");
}
```

What this example demonstrates:

- the macro creates `static` values `DARK` and `LIGHT`;
- `Themes::from_str` accepts strings case-insensitively;
- `Display` returns the theme name in lowercase;
- `Themes::default()` returns the first declared theme;
- the enum can be converted into `&'static Theme`;
- the full list of themes is available through `Themes::all_themes()`.

## Compatibility and limitations

### Scope requirements

The generated code refers to several symbols by their exact names. Before invoking the macro, the consuming crate must have the following imports in scope:

```rust
use serde::{Deserialize, Serialize};
use theme::{Colors, Theme};
```

This matters because the macro generates code using the direct names `Deserialize`, `Serialize`, `Theme`, and `Colors`, rather than fully qualified paths.

### Dependency on the neighboring workspace crate

In the current workspace, the types required by the macro are provided by the `theme` crate located at `../sss-lib/theme`.

That crate defines:

- [`Theme`](sss-lib/theme/src/lib.rs:13)
- `Colors`
- the `Shade` trait

If you want to call trait methods directly on generated static [`Theme`](sss-lib/theme/src/lib.rs:13) values, such as `DARK.get_colors()` or `DARK.font()`, also import `theme::Shade`.

### Current DSL limitations

The macro is designed for a strictly defined format and does not attempt to be a free-form configuration parser:

- both `colors` and `font` blocks are required;
- section order is fixed;
- field order inside sections is fixed;
- unexpected tokens after `font` result in an error;
- an empty macro invocation results in a parse error.

This contract makes code generation deterministic and predictable, but it must be taken into account when writing the input DSL.
