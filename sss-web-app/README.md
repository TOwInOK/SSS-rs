# SSS Web App

An interactive web application for creating and editing developer cards using Rust and the Leptos framework.

## 📋 Description

SSS Web App is a client-side web application that provides a convenient interface for creating professional developer portfolios. The application allows you to:

- Create and edit developer cards
- Choose from multiple themes (castle, github, umbrella)
- Preview cards in real-time
- Save and load configurations
- Export cards in various formats
- Use local storage for saving settings

## 🚀 Installation and running

### Requirements

- Rust (version specified in `rust-toolchain.toml`)
- Node.js (for Tailwind CSS)
- Trunk CLI

### Installing dependencies

```bash
# Installing Trunk
cargo install trunk

# Installing wasm-pack (optional)
cargo install wasm-pack
```

### Running in development mode

```bash
cd sss-web-app
trunk serve --open
```

The application will be available at `http://localhost:3000`

### Building for production

```bash
trunk build --release
```

Built files will be in the `dist/` directory

## 🏗️ Project structure

```
sss-web-app/
├── Cargo.toml              # Project configuration and dependencies
├── index.html              # HTML template with loading screen
├── public/                 # Static files
│   ├── favicon.ico        # Site icon
│   ├── index.css          # Main styles
│   └── styles.scss        # SCSS styles for Tailwind
└── src/
    ├── main.rs            # Application entry point
    ├── lib.rs             # Main Leptos module
    ├── tools.rs           # Helper functions
    ├── components/        # Leptos components
    │   ├── card_viewer.rs         # Card viewer component
    │   ├── settings_builder.rs    # Settings builder
    │   ├── tool_bar.rs            # Toolbar
    │   ├── toster.rs              # Notification component
    │   └── reusable_components/  # Reusable components
    └── pages/               # Application pages
        ├── home.rs         # Home page
        ├── card_editor.rs  # Card editor
        └── not_found.rs    # 404 page
```

## 🎨 Main components

### Reusable components
- **Button** — styled button
- **Input** — text input field
- **Textarea** — multiline input field
- **Selector** — dropdown list
- **Section** — section with header
- **Stack** — container for vertical element arrangement
- **ScrollableBox** — scrollable container

### Main components
- **CardViewer** — component for displaying developer cards
- **SettingsBuilder** — settings builder for editing card parameters
- **ToolBar** — toolbar with control buttons
  - **SaveButton** — saving configuration
  - **LoadButton** — loading configuration
  - **DownloadButton** — downloading card
  - **DropButton** — resetting settings
  - **RestoreButton** — restoring settings
- **Toster** — notification system

### Pages

- **Home** — main page with card selection and editing
- **CardEditor** — card editing page
- **NotFound** — page for non-existent routes

## ⚙️ Configuration

### Trunk

Trunk configuration is in `Cargo.toml`:

```toml
[package.metadata.trunk]
target = "index.html"
dist = "dist"
public_url = "/"
filehash = true
inject_scripts = true

[package.metadata.trunk.serve]
addresses = ["0.0.0.0"]
port = 3000
open = false

[package.metadata.trunk.tools]
tailwindcss = "4.1.0"

[package.metadata.trunk.build]
minify = "on_release"
```

## 🌐 Browser support

The application uses modern Web APIs and supports:
- Chrome/Edge (latest versions)
- Firefox (latest versions)
- Safari (latest versions)

WebAssembly and ES6+ support is required.

## 🔧 Development

### Adding new components

1. Create a component file in `src/components/`
2. Implement the component using Leptos API
3. Export the component in `src/components/mod.rs`

### Adding new pages

1. Create a page file in `src/pages/`
2. Implement the page using Leptos Router
3. Add the route in `src/main.rs`

### Styling

- Use Tailwind CSS classes for styling
- Add additional styles in `public/styles.scss`
- Global styles are in `public/index.css`


**Quick links:**
- [SSS-rs](../) — main project repository
- [sss-cli](../sss-cli/) — CLI tool for generating cards
- [sss-std](../sss-std/) — SSS standard library
- [sss-lib](../sss-lib/) — SSS core libraries
- [theme-generator](../theme-generator/) — theme generator
- [html-layouts-derive](../html-layouts-derive/) — HTML layout macros
- [icon-derive](../icon-derive/) — icon macros

## 📄 License

This project is part of SSS-rs and is distributed under the same license as the main project. See the `LICENSE` file in the repository root for details.

## 👨‍💻 Author

**TOwInOK**

## 📚 Additional documentation

- [Leptos Documentation](https://docs.rs/leptos/)
- [Trunk Documentation](https://trunkrs.dev/)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [SSS-rs Documentation](../README.md)

---

**Created with ❤️ in Rust and Leptos**
