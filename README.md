SSS-rs (Skill, Slick, Style) is a library and CLI tool for creating stylish developer cards.

## Available languages README
[RU](README_ru.md) | [EN](README_ru.md)

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
Allows you to host generated pages with hot reload for themes and layouts.

## Usage

```bash
sss_cli [OPTIONS] <COMMAND>
```

### Global Options

- `-c, --config <PATH>` - path to config file (default: config.toml)
- `-t, --theme <THEME>` - theme selection [possible values: umbrella, rose-pine, groove-box, dracula]
- `-l, --layout <LAYOUT>` - layout selection [possible values: umbrella, castle, github]
- `-o, --out <FILE>` - output HTML filename (default: sss-rs.html)
- `--tracing <TRACING>` - Log level [default: info] [possible values: info, trace, debug, error, warn]
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
  -a, --address <ADDRESS>   Web server address [default: 0.0.0.0:8081]
```

#### gen - Generate HTML
```bash
sss_cli gen
```

### Usage Examples

```bash
# Create new TOML configuration
sss_cli new
# or
# Create JSON configuration
sss_cli new --config-type json

# Start development server with auto-reload
sss_cli run --watch --serve
# or
sss_cli run -w -s

# Additional options

# Generate HTML with custom output file
sss_cli -o portfolio.html gen

# Commands below disable hot reload for applied value
# Generate HTML with theme
sss_cli -t dracula gen

# Generate HTML with layout
sss_cli -l github gen
```

## How to build CLI

```bash
git clone https://github.com/TOwInOK/SSS-rs
cd SSS-rs
cargo build -r -p sss_rs
mv target/release/sss_cli sss-cli
./sss-cli
```

## How to run file downloaded from github on macos
```sh
xattr -rd com.apple.quarantine name_of_file
./name_of_file
```

## License
[Apache 2.0](LICENSE)

## Contributing
If you want to add a new theme or layout, create an Issue!

# Example
![Card Example](.content/umbrella.umbrella.jpeg)
