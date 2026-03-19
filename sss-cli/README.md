# sss-cli

CLI tool for generating HTML/PNG/PDF using SSS-rs themes and layouts. Supports hot reload, web server, and file watching.

## Features

- **Configuration Generation**: Create new TOML or JSON configuration files with example data
- **HTML Generation**: Generate static HTML from configuration
- **PNG Generation**: Convert HTML to PNG images (requires headless Chrome/Chromium)
- **PDF Generation**: Convert HTML to PDF documents (requires headless Chrome/Chromium)
- **Hot Reload**: Watch configuration file and automatically refresh outputs
- **Web Server**: Serve generated content with optional API documentation
- **Service Management**: Enable/disable individual services (html, png, pdf, json, health, share, api)

## Installation

Build from source:
```bash
cargo build --release -p sss_cli
```

The binary will be available at `target/release/sss_cli`.

## Usage

### Global Options

- `-c, --config <PATH>` - Path to configuration file (default: `config.toml`)
- `-t, --theme <THEME>` - Theme selection (umbrella, rose-pine, groove-box, dracula)
- `-l, --layout <LAYOUT>` - Layout selection (umbrella, castle, github)
- `--tracing <LEVEL>` - Log level (info, trace, debug, error, warn)

### Commands

#### `new` - Generate new configuration

Create a new configuration file with example data.

```bash
sss_cli new [OPTIONS]

Options:
  -t, --type <TYPE>    Configuration format (default: toml) [possible values: json, toml]
  -b, --base64 <STRING>  Decode base64 string to TOML configuration
```

Examples:
```bash
# Create default TOML configuration
sss_cli new

# Create JSON configuration
sss_cli new --type json

# Create from base64 string
sss_cli new --base64 "c2Vz..."
```

#### `run` - Run web server and/or file watcher

Start the development server with optional file watching.

```bash
sss_cli run [OPTIONS]

Options:
  -w, --watch               Watch for configuration changes
  -s, --serve               Start web server
  -a, --address <ADDRESS>   Web server address (default: 0.0.0.0:8081)
  --html                    Enable HTML service
  --png                     Enable PNG generation service
  --pdf                     Enable PDF generation service
  --json                    Enable JSON settings service
  --health                  Enable health check service
  --share                   Enable base64 sharing service
  --api                     Enable API documentation service
```

Examples:
```bash
# Start server with hot reload
sss_cli run --watch --serve

# Start with specific services
sss_cli run -w -s --html --png --json --api

# Custom address
sss_cli run -s --address 127.0.0.1:3000
```

#### `gen` - Generate HTML/PNG/PDF files

Generate output files from configuration.

```bash
sss_cli gen [OPTIONS]

Options:
  -t, --type <TYPE>  Output type (default: html) [possible values: html, png, pdf]
  -o, --out <NAME>    Output name (default: sss-rs)
```

Examples:
```bash
# Generate HTML
sss_cli gen

# Generate PNG
sss_cli gen --type png

# Custom output name
sss_cli gen -o my-card --type pdf
```

## Available Routes

When the web server is running, the following routes are available:

- `/` - HTML card page (always available)
- `/png` - PNG image (requires `--png` flag)
- `/pdf` - PDF document (requires `--pdf` flag)
- `/json` - Current settings as JSON (requires `--json` flag)
- `/health` - Health check endpoint (requires `--health` flag)
- `/share` - Base64 encoded settings (requires `--share` flag)
- `/api` - API documentation (requires `--api` flag)

## Dependencies

- **headless_chrome**: Required for PNG/PDF generation
- **notify**: Cross-platform file system watching
- **axum**: Web server framework
- **tokio**: Async runtime
- **clap**: Command-line argument parsing

## Development

Run with debug logging:
```bash
RUST_LOG=debug sss_cli run --watch --serve
```

## License

Apache-2.0
