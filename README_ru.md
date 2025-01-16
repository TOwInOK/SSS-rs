# SSS-rs

SSS-rs (Skill, Slick, Style) - это библиотека и инструмент командной строки для создания стильных визиток разработчика.

## Доступные языки для readme
[RU](README_ru#SSS-rs) | [EN](README_ru#SSS-rs)

## Библиотека

### Основные компоненты

1. **Конфигурация** (`sss-core`)
   - Структуры данных для информации о разработчике

2. **Темы** (`render`, trait `Shade`)
   - Настраиваемая цветовая схема
   - Встроенные темы:
     - Umbrella (по умолчанию)
     - Rosé Pine
     - GrooveBox
     - Dracula

3. **Макеты** (`render`, trait `Render`)
   - Система шаблонов на основе Tera
   - Поддержка HTML и CSS
   - Адаптивный дизайн

### Использование в качестве библиотеки

```rust
use sss_core::Settings;
use sss_std::themes::Themes;
use sss_std::layouts::Layouts;

// Создание конфигурации
let settings = Settings::default();

// Выбор темы и макета
let theme = Themes::Umbrella;
let layout = Layouts::Umbrella;

// Генерация HTML
let html = layout.to_layout(&settings, &theme.into())
    .finalize()
    .unwrap();
```

# SSS-rs CLI

CLI инструмент для генерации HTML с использованием тем и макетов SSS-rs.

## Использование

```bash
sss_cli [OPTIONS] <COMMAND>
```

### Общие опции

- `-c, --config <PATH>` - путь к файлу конфигурации (по умолчанию: config.toml)
- `-t, --theme <THEME>` - выбор темы оформления [возможные значения: umbrella, rose-pine, groove-box, dracula]
- `-l, --layout <LAYOUT>` - выбор макета [возможные значения: umbrella]
- `-o, --out <FILE>` - имя выходного HTML файла (по умолчанию: sss-rs.html)
- `-h, --help` - вывод справки
- `-V, --version` - вывод версии

### Команды

#### new - Создание новой конфигурации
```bash
sss_cli new [OPTIONS]

Опции:
  -c, --config-type <TYPE>    Формат конфигурации [по умолчанию: toml]
                             [возможные значения: json, toml]
```

#### run - Запуск веб-сервера
```bash
sss_cli run [OPTIONS]

Опции:
  -w, --watch                Отслеживание изменений конфигурации
  -s, --serve               Запуск веб-сервера
  -a, --address <ADDRESS>   Адрес веб-сервера [по умолчанию: 127.0.0.1:8081]
```

#### gen - Генерация HTML
```bash
sss_cli gen
```

### Примеры использования

```bash
# Создание новой конфигурации в формате TOML
sss_cli new

# Создание конфигурации в формате JSON
sss_cli new --config-type json

# Запуск сервера разработки с автообновлением
sss_cli run --watch --serve

# Генерация HTML с пользовательской темой
sss_cli -t dracula gen

# Генерация HTML с указанием выходного файла
sss_cli -o portfolio.html gen
```

## Лицензия
[Apache 2.0](LICENSE)

## Улучшение продукта
Если вы хотите добавить новую тему или шаблон, создайте issue!

# Пример
![Пример карточки](.content/umbrella.umbrella.jpeg)
