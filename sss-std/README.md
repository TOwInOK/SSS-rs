# SSS-std

SSS-rs standard library for generating developer cards in various formats.

## 📋 Description

SSS-std provides the main tools and components for creating professional developer cards. The library includes:

- **HTML templates** — ready-to-use templates for rendering cards using Tera
- **Converters** — converting cards to various formats (HTML, PDF, PNG)
- **Themes** — a collection of preset color schemes and fonts
- **Layouts** — a system for creating and managing HTML layouts

## 🚀 Features

### Supported formats

| Format | Description | Feature |
|--------|-------------|---------|
| **HTML** | Static HTML generation with embedded styles | Basic |
| **PDF** | Export to PDF document | `image_generation` |
| **PNG** | PNG image creation | `image_generation` |

### Themes

The library includes 17 preset themes:

#### Classic themes
- **UMBRELLA** — dark theme with purple accents
- **ROSE_PINE** — soft Rose Pine style theme
- **GROOVEBOX** — dark theme with orange accents
- **DRACULA** — popular Dracula theme

#### Catppuccin (13 variations)
- **MAUVE** — lavender accent
- **FLAMINGO** — pink accent
- **ROSEWATER** — light pink accent
- **PINK** — pink accent
- **RED** — red accent
- **MAROON** — maroon accent
- **PEACH** — peach accent
- **YELLOW** — yellow accent
- **GREEN** — green accent
- **TEAL** — teal accent
- **SKY** — sky blue accent
- **SAPPHIRE** — sapphire accent
- **BLUE** — blue accent
- **LAVENDER** — lavender accent

### HTML templates

The library includes three ready-to-use templates:

1. **Castle** — stylish template with dark background
2. **GitHub** — minimalist GitHub-style template
3. **Umbrella** — elegant template with accents

## 📦 Installation

Add dependency to `Cargo.toml`:

```toml
[dependencies]
sss_std = { path = "../sss-std" }
```

### Features

```toml
[dependencies]
sss_std = { path = "../sss-std", features = ["full"] }
```

Available features:
- `default` — basic functionality (without image generation)
- `full` — includes all features (`image_generation` + `utoipa`)
- `image_generation` — includes PDF and PNG generation support
- `utoipa` — includes OpenAPI documentation support

## 🏗️ Project structure

```
sss-std/
├── Cargo.toml              # Конфигурация проекта
├── card.html               # Пример сгенерированной HTML карточки
├── card.pdf                # Пример сгенерированной PDF карточки
├── card.png                # Пример сгенерированной PNG карточки
├── src/
│   ├── lib.rs              # Основной модуль библиотеки
│   ├── prelude.rs          # Переэкспорт часто используемых типов
│   ├── themes.rs           # Предустановленные темы оформления
│   ├── tools.rs            # Вспомогательные инструменты
│   ├── converter/          # Конвертеры форматов
│   │   ├── pdf.rs          # Конвертер в PDF
│   │   └── png.rs          # Конвертер в PNG
│   └── layouts/            # HTML-макеты
│       └── html_tera/      # Шаблоны на Tera
│           ├── builder.rs          # Билдер для карточек
│           ├── final_builder.rs    # Финальный билдер
│           ├── html_meta.rs        # Мета-теги HTML
│           ├── tools.rs            # Инструменты для шаблонов
│           ├── templates/          # Шаблоны карточек
│           │   ├── castle/
│           │   │   ├── card.html.tera
│           │   │   └── card.limitations.ron
│           │   ├── github/
│           │   │   ├── card.html.tera
│           │   │   └── card.limitations.ron
│           │   └── umbrella/
│           │       ├── card.html.tera
│           │       └── card.limitations.ron
│           └── final_templates/    # Финальные шаблоны
│               └── standart/
│                   └── default_layout.html.tera
└── tests/
    └── layout.rs           # Тесты макетов
```

## 🔧 Usage

### Basic usage

```rust
use sss_std::prelude::*;

// Creating a card
let card = Card::new(/* ... */);

// Rendering to HTML
let html = render_html(&card, "umbrella")?;

// Saving to file
std::fs::write("card.html", html)?;
```

### PDF generation (feature: `image_generation`)

```rust
use sss_std::converter::pdf::to_pdf;

// Converting HTML to PDF
let pdf_bytes = to_pdf(&html_content)?;

// Saving PDF
std::fs::write("card.pdf", pdf_bytes)?;
```

### PNG generation (feature: `image_generation`)

```rust
use sss_std::converter::png::to_png;

// Converting HTML to PNG
let png_bytes = to_png(&html_content)?;

// Saving PNG
std::fs::write("card.png", png_bytes)?;
```

### Working with themes

```rust
use sss_std::themes::UMBRELLA;

// Using a preset theme
let theme = UMBRELLA::theme();

// Getting colors
let text_color = theme.colors.text;
let bg_color = theme.colors.background;

// Getting font
let font_name = theme.font.name;
let font_link = theme.font.link;
```

### Using HTML templates

```rust
use sss_std::layouts::html_tera::builder::CardBuilder;

// Creating a builder
let builder = CardBuilder::new("umbrella")
    .with_user(user)
    .with_skills(skills)
    .with_links(links);

// Rendering the card
let html = builder.render()?;
```

## 🎨 Creating custom themes

To create a custom theme, use the `generate_theme!` macro:

```rust
use sss_std::themes::generate_theme;

generate_theme! {
    MY_THEME {
        colors {
            text: "#ffffff",
            background: "#1a1a1a",
            accent: "#ff6b6b",
            border: "#333333",
        },
        font {
            name: "Fira Code",
            link: "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&display=swap"
        }
    }
}
```

## 📝 Template system

### Limitations file (`.limitations.ron`)

Each template has a limitations file that defines:
- Maximum number of elements of each type
- Text length limitations
- Supported features

Example:
```ron
(
    max_skills: 10,
    max_links: 8,
    max_projects: 5,
    max_description_length: 200,
)
```

### Tera templates

Templates use Tera syntax:

```tera
<div class="card">
    <h1>{{ user.nickname }}</h1>
    <p>{{ user.description }}</p>
    {% for skill in skills %}
        <span>{{ skill.name }}</span>
    {% endfor %}
</div>
```

## 🔌 Dependencies

### Main dependencies

- **render** — SSS rendering module
- **theme** — SSS theme module
- **sss_core** — SSS core library
- **html-layouts-derive** — derive macro for generating layouts
- **theme-generator** — theme generator
- **tera** — template engine
- **encre-css** — CSS generator
- **serde** — serialization/deserialization
- **ron** — data format

### Optional dependencies

- **image** (feature: `image_generation`) — image processing
- **headless_chrome** (feature: `image_generation`) — headless browser rendering
- **utoipa** (feature: `utoipa`) — OpenAPI documentation generation

## 🧪 Testing

Running tests:

```bash
cargo test
```

Running tests with full features:

```bash
cargo test --features full
```

## 📚 Examples

Usage examples can be found in the `tests/` directory:

```bash
cargo test --test layout --features full
```

## 🔗 Related projects

- [SSS-rs](../) — main project repository
- [sss-cli](../sss-cli/) — CLI tool for generating cards
- [sss-web-app](../sss-web-app/) — web application for creating cards
- [sss-lib/sss-core](../sss-lib/sss-core/) — SSS core library
- [theme-generator](../theme-generator/) — theme generator

## 📄 License

This project is part of SSS-rs and is distributed under the same license as the main project. See the `LICENSE` file in the repository root for details.

## 👨‍💻 Author

**TOwInOK**

## 🤝 Contributing

If you want to contribute to the development:

1. Fork the repository
2. Create a branch for your feature (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📚 Additional documentation

- [Tera Documentation](https://tera.netlify.app/)
- [Serde Documentation](https://serde.rs/)
- [RON Documentation](https://github.com/ron-rs/ron)

---

**Created with ❤️ in Rust**
