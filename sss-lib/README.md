# SSS-lib

A collection of core libraries for the SSS-rs project, providing basic functionality for working with developer cards.

## 📋 Overview

SSS-lib is a set of libraries that form the foundation of the SSS-rs ecosystem. Each library performs a specific task:

- **parser** — parsing and saving data in various formats
- **render** — rendering and layout management
- **sss-core** — core data types and structures
- **theme** — theming and styling system

## 📦 Structure

```
sss-lib/
├── parser/          # Data parsing module
├── render/          # Rendering module
├── sss-core/        # Core library
└── theme/           # Theming module
```

## 🚀 Libraries

### 1. Parser

Module for parsing and saving data in various formats.

#### Features

- Format support: **TOML**, **JSON**, **RON**
- Automatic format detection by file extension
- Convenient `Loader` and `Saver` traits for working with files
- Error handling with detailed messages

#### Usage

```rust
use parser::prelude::*;

// Parsing from file (automatic format detection)
let data: MyType = parse("config.toml")?;

// Parsing from specific format
let data: MyType = parse_toml("config.toml")?;
let data: MyType = parse_json("config.json")?;
let data: MyType = parse_ron("config.ron")?;

// Saving to file (automatic format detection)
save("output.json", &data)?;

// Saving to specific format
save_toml("output.toml", &data)?;
save_json("output.json", &data)?;
save_ron("output.ron", &data)?;

// Using traits
impl Loader for MyStruct {}
impl Saver for MyStruct {}

let loaded = MyStruct::load::<MyType>("config.toml")?;
MyStruct::save("output.json", &data)?;
```

#### Dependencies

- `serde` — serialization/deserialization
- `serde_json` — JSON support
- `toml` — TOML support
- `ron` — RON support
- `thiserror` — error handling

---

### 2. Render

Module for organizing drawing and rendering utilities.

#### Features

- Layout management and positioning
- Theme and style configuration
- Content generation and formatting
- Output formatting and rendering

#### Main components

```rust
use render::prelude::*;

// Trait for components
pub trait Component<T: Display + Clone>
where
    Self: Layout<T> + Render<T>,
{
}
```

#### Dependencies

- `sss_core` — core data types
- `serde` — serialization
- `serde_json` — JSON support
- `encre-css` — CSS generation

---

### 3. SSS-core

Core library containing data types and structures for developer cards.

#### Core types

##### `Data`

Main data structure for SSS:

```rust
pub struct Data {
    pub layout: LayoutData,
}
```

##### `LayoutData`

Structure with layout data:

```rust
pub struct LayoutData {
    pub user: User,                    // User information
    pub specifications: Vec<String>,    // Specializations
    pub about: String,                 // User description
    pub repos: Vec<Project>,           // Repositories
    pub socials: Vec<Link>,            // Social networks
    pub skills: Vec<Skill>,            // Skills
}
```

##### `User`

User information:

```rust
pub struct User {
    pub name: String,                      // Name
    pub current_nickname: Nickname,         // Current nickname
    pub prevision_nicknames: Vec<Nickname>, // Previous nicknames
}
```

##### `Skill`

Skill information:

```rust
pub struct Skill {
    pub skill: String,          // Skill name
    pub projects: Vec<Project>,  // Projects with this skill
    pub since: Since,           // Usage period
    pub main: bool,             // Main skill
    pub repo_link: Link,        // Repository link
}
```

##### `Project`

Project information:

```rust
pub struct Project {
    pub name: String,  // Project name
    pub link: Link,    // Project link
}
```

##### `Link`

Resource link:

```rust
pub struct Link {
    pub provider: String,  // Provider (github, telegram, etc.)
    pub url: String,        // URL
}
```

##### `Since`

Time period:

```rust
pub struct Since {
    pub start: u32,  // Start year
    pub end: u32,    // End year (0 = today)
}
```

#### Limitations

The library includes a limitation system for data validation:

```rust
pub struct LayoutLimitations {
    pub user: Option<UserLimitations>,
    pub specifications_count: (usize, usize),
    pub about: Option<usize>,
    pub repositories: (usize, usize),
    pub socials: Option<usize>,
    pub skills: (usize, SkillLimitation),
}
```

#### Features

- `default` — basic functionality
- `full` — includes all features (`utoipa` + `leptos`)
- `utoipa` — OpenAPI documentation support
- `leptos` — Leptos integration for web applications

#### Usage

```rust
use sss_core::prelude::*;

// Creating data
let data = Data {
    layout: LayoutData {
        user: User {
            name: "John Doe".to_string(),
            current_nickname: Nickname {
                name: "johndoe".to_string(),
                pronounce: "john-doe".to_string(),
            },
            prevision_nicknames: vec![],
        },
        specifications: vec!["Backend Development".to_string()],
        about: "Full-stack developer".to_string(),
        repos: vec![],
        socials: vec![],
        skills: vec![],
    },
};

// Saving to file
data.save("config.toml")?;

// Loading from file
let loaded: Data = Data::load("config.toml")?;
```

#### Dependencies

- `serde` — serialization/deserialization
- `parser` — file parsing
- `tabler-icon-definer` — icons
- `utoipa` (optional) — OpenAPI documentation
- `leptos` (optional) — web framework

---

### 4. Theme

Module for theme management and styling.

#### Core types

##### `Theme`

Main theme structure:

```rust
pub struct Theme {
    pub colors: Colors,
    pub font: (&'static str, &'static str),  // (font name, URL)
}
```

##### `Colors`

Theme color palette:

```rust
pub struct Colors {
    pub text: Color,        // Main text color
    pub background: Color,   // Background color
    pub accent: Color,       // Accent color
    pub border: Color,       // Border color
}
```

#### `Shade` trait

Trait for implementing themes:

```rust
pub trait Shade: Sync + Send {
    fn get_colors(&self) -> &Colors;
    fn font(&self) -> (&'static str, &'static str);
    fn get_encre_css_config(&self) -> Config;
}
```

#### Usage

```rust
use theme::{Theme, Colors, Shade};

// Creating a theme
let theme = Theme {
    colors: Colors {
        text: "#ffffff",
        background: "#1a1a1a",
        accent: "#ff6b6b",
        border: "#333333",
    },
    font: ("Fira Code", "https://fonts.googleapis.com/css2?family=Fira+Code&display=swap"),
};

// Getting colors
let colors = theme.get_colors();
let text_color = colors.text;

// Getting font
let (font_name, font_url) = theme.font();

// Getting CSS configuration
let css_config = theme.get_encre_css_config();
```

#### Dependencies

- `serde` — serialization
- `encre-css` — CSS generation

## 🔧 Installation

Each library can be installed separately:

```toml
[dependencies]
parser = { path = "sss-lib/parser" }
render = { path = "sss-lib/render" }
sss_core = { path = "sss-lib/sss-core" }
theme = { path = "sss-lib/theme" }
```

Or all together:

```toml
[dependencies]
sss_lib = { path = "sss-lib" }
```

## 📚 Examples

### Complete example of creating a developer card

```rust
use sss_core::prelude::*;
use parser::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creating data
    let data = Data {
        layout: LayoutData {
            user: User {
                name: "John Doe".to_string(),
                current_nickname: Nickname {
                    name: "johndoe".to_string(),
                    pronounce: "john-doe".to_string(),
                },
                prevision_nicknames: vec![],
            },
            specifications: vec![
                "Backend Development".to_string(),
                "Full-Stack Development".to_string(),
            ],
            about: "Passionate developer with 5+ years of experience".to_string(),
            repos: vec![Project {
                name: "awesome-project".to_string(),
                link: Link {
                    provider: "github".to_string(),
                    url: "https://github.com/johndoe/awesome-project".to_string(),
                },
            }],
            socials: vec![Link {
                provider: "github".to_string(),
                url: "https://github.com/johndoe".to_string(),
            }],
            skills: vec![Skill {
                skill: "Rust".to_string(),
                projects: vec![],
                since: Since { start: 2019, end: 0 },
                main: true,
                repo_link: Link {
                    provider: "crates".to_string(),
                    url: "https://crates.io/users/johndoe".to_string(),
                },
            }],
        },
    };

    // Saving to file
    data.save("my_card.toml")?;

    println!("Card saved successfully!");

    Ok(())
}
```

## 🔗 Related projects

- [SSS-rs](../) — main project repository
- [sss-std](../sss-std/) — SSS standard library
- [sss-cli](../sss-cli/) — CLI tool
- [sss-web-app](../sss-web-app/) — web application
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

---

**Created with ❤️ in Rust**
