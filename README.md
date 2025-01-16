# SSS-rs

SSS-rs (Skill, Slick, Style) is a library and CLI tool for creating stylish developer cards.

## Available languages readme
[RU](README_ru#SSS-rs) | [EN](README_ru#SSS-rs)

## Library

### Core Components

1. **Configuration** (`sss-core`)
   - Data structures for developer information

2. **Themes** (`render`, trait `Shade`)
   - Customizable color schemes
   - Built-in themes:
     - Umbrella (default)
     - Rosé Pine
     - GrooveBox
     - Dracula

3. **Layouts** (`render`, trait `Render`)
   - Tera-based templating system
   - HTML and CSS support
   - Responsive design

### Library Usage

```rust
use sss_core::Settings;
use sss_std::themes::Themes;
use sss_std::layouts::Layouts;

// Create configuration
let settings = Settings::default();

// Choose theme and layout
let theme = Themes::Umbrella;
let layout = Layouts::Umbrella;

// Generate HTML
let html = layout.to_layout(&settings, &theme.into())
    .finalize()
    .unwrap();
```

# SSS-rs CLI

CLI tool for generating HTML using SSS-rs themes and layouts.

## Usage

```bash
sss_cli [OPTIONS] <COMMAND>
```

### Global Options

- `-c, --config <PATH>` - path to config file (default: config.toml)
- `-t, --theme <THEME>` - theme selection [possible values: umbrella, rose-pine, groove-box, dracula]
- `-l, --layout <LAYOUT>` - layout selection [possible values: umbrella]
- `-o, --out <FILE>` - output HTML filename (default: sss-rs.html)
- `-h, --help` - print help
- `-V, --version` - print version

### Commands

#### new - Generate new config
```bash
sss_cli new [OPTIONS]

Options:
  -c, --config-type <TYPE>    Configuration format [default: toml]
                             [possible values: json, toml]
```

#### run - Start development server
```bash
sss_cli run [OPTIONS]

Options:
  -w, --watch                Watch for config changes
  -s, --serve               Start web server
  -a, --address <ADDRESS>   Web server address [default: 127.0.0.1:8081]
```

#### gen - Generate HTML
```bash
sss_cli gen
```

### Usage Examples

```bash
# Create new TOML configuration
sss_cli new

# Create JSON configuration
sss_cli new --config-type json

# Start development server with auto-reload
sss_cli run --watch --serve

# Generate HTML with custom theme
sss_cli -t dracula gen

# Generate HTML with custom output file
sss_cli -o portfolio.html gen
```

## License
[Apache 2.0](LICENSE)

## Contributing
If you want to add a new theme or layout, create an Issue!

# Example
[Card Example](.content/umbrella.umbrella.jpeg)
