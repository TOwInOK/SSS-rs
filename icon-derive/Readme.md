# icon-derive

A Rust procedural macro crate for embedding Tabler SVG icons into your binary at compile time.

## Features

- Downloads SVG icons during compilation
- Caches icons locally for 30 days
- Supports Tabler icons from unpkg.com
- Generates type-safe enum for icon access
- Customizable icon names
- Optional derive support for additional traits

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
icon-derive = "2"
```

## Usage

### Basic Usage

```rust
use icon_derive::tabler_icon;

tabler_icon!(
    brand_github[outline, filled],
    user[outline]
);

// Generates:
// - Tabler::OUTLINE_BRAND_GITHUB
// - Tabler::FILLED_BRAND_GITHUB
// - Tabler::OUTLINE_USER
```

### Custom Names

```rust
tabler_icon!(
    #[name = "github"]
    brand_github[outline, filled]
);

// Generates:
// - Tabler::OUTLINE_GITHUB
// - Tabler::FILLED_GITHUB
```

### Additional Derives

```rust
tabler_icon!(
    #[derive(serde::Serialize)]
    #[name = "github"]
    brand_github[outline, filled]
);
```

### Using Generated Icons

```rust
let icon = Tabler::OUTLINE_GITHUB;

// Get SVG content
let svg = icon.as_str();

// Get icon name without style prefix
let name = icon.as_str_merget();

// Get full icon name with style prefix
let full_name = icon.to_string();

// Get all available icons
let all_icons = Tabler::all_icons();
```

## Icon Naming

To find the correct icon names, visit [Tabler Icons](https://tabler.io/icons).

## Caching

Icons are cached in `target/icon_cache` for 30 days to avoid unnecessary downloads.

## Notes

- The first icon in the list becomes the default variant
- Icons are downloaded from unpkg.com during compilation
- Supports both outline and filled styles for Tabler icons

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
