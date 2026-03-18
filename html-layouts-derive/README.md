# html-layouts-derive

`html-layouts-derive` is the proc-macro crate that turns layout directories in the SSS workspace
into strongly typed Rust enums used by the HTML renderer.

## Purpose

The crate exists to remove handwritten enum maintenance for HTML layouts. Instead of manually
updating Rust code every time a layout is added, the macro scans layout folders at compile time and
generates the layout types and helper implementations automatically.

## Provided proc-macros

The crate currently provides one proc-macro:

- `generate_layouts!(templates_dir, default_layouts_dir)`

The macro expects exactly two string literals:

1. a directory with card layout subdirectories;
2. a directory with default outer-layout subdirectories.

## How template scanning works

During macro expansion the crate resolves both paths relative to the consuming crate's
`CARGO_MANIFEST_DIR`. It then scans only the immediate child directories of each root.

### Card layout directories

Each card layout directory must contain:

- `card.html.tera` — the Tera template for the card itself;
- `card.limitations.ron` — the parsed `LayoutLimitations` declaration for that layout.

### Default layout directories

Each default layout directory must contain:

- `default_layout.html.tera` — the outer Tera template that wraps rendered card HTML.

Directory names are normalized in two ways:

- lowercase names are used for `Display` / `FromStr` string APIs;
- uppercase names are used for generated Rust enum variants.

The discovered directories are sorted by normalized lowercase name so the generated code stays
deterministic across builds and filesystems.

## What the macro generates

For card layouts the macro generates:

- the `HtmlLayouts` enum;
- `Layout<String>` implementation;
- `Limitations` implementation;
- `Display` and `FromStr` implementations;
- `HtmlLayouts::all()`;
- `Default` for `HtmlLayouts` using the first layout after deterministic sorting;
- per-layout constants with embedded `LayoutLimitations` values.

For default layouts the macro generates:

- the `DefaultTemplates` enum;
- `Layout<String>` implementation;
- `Limitations` implementation that returns `None`;
- `Display` and `FromStr` implementations;
- `DefaultTemplates::all()`.

All template sources are embedded into the generated code as string literals, so no runtime layout
discovery is required.

## Relationship with `sss-std` and layouts

This crate is consumed by `sss-std`, specifically in `sss-std/src/layouts/html_tera/mod.rs`:

```rust
use html_layouts_derive::generate_layouts;

generate_layouts!(
    "src/layouts/html_tera/templates",
    "src/layouts/html_tera/final_templates"
);
```

That invocation generates the `HtmlLayouts` and `DefaultTemplates` enums that `sss-std` re-exports
from its layouts module and passes into the HTML/Tera rendering pipeline.

## Directory structure expectations and limitations

- Both macro arguments must be valid paths relative to the consuming crate.
- Each scanned root must contain at least one layout subdirectory.
- Only immediate child directories are treated as layouts.
- Card layout directories must contain both required files.
- Default layout directories must contain their required wrapper template.
- Directory names must be valid UTF-8.
- Directory names must remain unique after lowercase normalization.
- Directory names must also remain unique after conversion to uppercase Rust identifiers.

If any of these expectations are violated, the macro emits a compile-time error with the offending
path or directory name.

## Intended usage scope

`html-layouts-derive` is an internal build-time crate for the SSS layout system. It is most useful
when paired with the directory conventions already used by `sss-std::layouts::html_tera`.
