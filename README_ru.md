# SSS-rs

SSS-rs (Skill, Slick, Style) - это библиотека и инструмент командной строки для создания стильных визиток разработчика.

## Доступные языки README
[RU](README_ru.md) | [EN](README.md)

## Библиотека

### Основные компоненты

1. **Конфигурация** (`sss-core`)
   - Структуры данных для информации о разработчике

2. **Темы** (`render`, trait `Shade` or structure `Theme`)
   - Настраиваемые цветовые схемы
   - Встроенные темы:
     - Umbrella (по умолчанию)
     - Rosé Pine
     - GrooveBox
     - Dracula

3. **Макеты** (`render`, trait `Layout` + `Finalize`)
   - Система шаблонов на основе Tera
   - Поддержка HTML и [TailwindCSS*](https://crates.io/crates/encre-css)
   - Адаптивный дизайн

### Использование библиотеки

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

CLI инструмент для генерации HTML/PNG/PDF с использованием тем и макетов SSS-rs.
Позволяет запускать сгенерированную страницу с автоматическим обновлением при изменении тем и макетов.

## Зависимости
- PNG/PDF
  - chromium/chrome (режим headless_chrome)
    - Если у вас его нет, эти функции не будут работать, но все остальное будет работать.

## Использование

```bash
sss_cli [OPTIONS] <COMMAND>
```

### Общие параметры

- `-c, --config <PATH>` - путь к файлу конфигурации (по умолчанию: config.toml)
- `-t, --theme <THEME>` - выбор темы оформления [возможные значения: umbrella, rose-pine, groove-box, dracula]
- `-l, --layout <LAYOUT>` - выбор макета [возможные значения: umbrella, castle, github]
- `-o, --out <FILE>` - имя выходного HTML файла (по умолчанию: sss-rs.html)
- `--tracing <TRACING>` - уровень логирования [по умолчанию: info] [возможные значения: info, trace, debug, error, warn]
- `-h, --help` - вывод справки
- `-V, --version` - вывод версии

### Команды

#### new - Создание новой конфигурации
```bash
sss_cli new [OPTIONS]

Параметры:
  -t, --type <TYPE>    Фармат настроек [по умолчанию: toml]
                           [возможные значения: json, toml]
```

#### run - Запуск сервера разработки
```bash
sss_cli run [OPTIONS]

Параметры:
  -w, --watch               Отслеживание изменений конфигурации
  -s, --serve               Запуск веб-сервера
  -a, --address <ADDRESS>   Адрес веб-сервера [по умолчанию: 0.0.0.0:8081]
  --html                    Запустить html-сервис [по умолчанию]
  --png                     Запуск сервиса генератора png
  --pdf                     Запуск сервиса генератора PDF
  --json                    Запустить JSON конвертер для службы настроек
  --Health                  Запустить heafcheck сервис
  --share                   Запуск base64 сервис
  --api                     Запустить rapidoc api сервис
```

### Доступные пути
- Api документация
  - Scalar API Page: /api
- Пути
  - Основная карточка (html) page: /
  - Png карточка: /png
  - Pdf карточка: /pdf
  - Json настройки экземпляра: /json
  - Base64 toml конфигурация : /share
  - Проверка здоровья сервера : /health

#### gen - Генерация HTML
```bash
sss_cli gen [OPTIONS]

Options:
  -t, --type <OUTPUT_TYPE>  выходной тип [по умолчанию: html] [возможные значения: html, png, pdf]
  -o, --out <OUTPUT_NAME>   выходное название [по умолчанию: sss-rs]
```

### Примеры использования

```bash
# Создание новой конфигурации в формате TOML
sss_cli new
# или
# Создание конфигурации в формате JSON
sss_cli new --config-type json

# Запуск сервера разработки с автообновлением
sss_cli run --watch --serve
# или (html по умолчанию)
sss_cli run -w -s
# Запуск с htmk, pnn, json и api роутами
sss_cli run -w -s --html --png --json --api

# Дополнительные опции

# Генерация HTML с указанием выходного файла
sss_cli -o portfolio.html gen

# Следующие команды отключают автообновление для применяемого значения
# Генерация HTML с указанной темой
sss_cli -t dracula gen

# Генерация HTML с указанным макетом
sss_cli -l github gen
```

## Как собрать CLI

```bash
git clone https://github.com/TOwInOK/SSS-rs
cd SSS-rs
cargo build -r -p sss_rs
mv target/release/sss_cli sss-cli
./sss-cli
```

## Как запустить файл скаченный с github на macos
```sh
xattr -rd com.apple.quarantine name_of_file
./name_of_file
```

# SSS-rs Web App

## Как запустить локально?
[вам необходимо установить trunk](https://trunkrs.dev/guide/getting-started/installation.html), затем запустите
```sh
cd sss-web-app
trunk serve --open
```
Эта команда запустит сервер на порту 3000 и откроет ваш браузер по умолчанию.

## Как построить свой собственный макет

[**Руководство**](How_to_construct_layout.md)

- SSS cli
  - На данный момент вы не можете использовать собственные шаблоны непосредственно в cli.
  Вы можете создать [Issue](https://github.com/TOwInOK/SSS-rs/issues/new?template=Blank+issue) для их добавления.
- SSS-lib
  - Вы можете создавать и использовать собственные шаблоны с помощью [HtmlTeraRender](sss-lib/sss-std/src/layouts/html_tera_builder.rs)
  или
  Создайте свою собственную реализацию с помощью трейтов: [Layout + Finalise](sss-lib/render/src/layout.rs)

## Лицензия
[Apache 2.0](LICENSE)

## Как внести свой вклад
Если вы хотите добавить новую тему или макет, создайте [Issue](https://github.com/TOwInOK/SSS-rs/issues/new?template=Blank+issue)!

# Пример
![Пример карточки](.content/umbrella.umbrella.jpeg)
