# SSS-rs

SSS-rs (Skill, Slick, Style) - это библиотека и инструмент командной строки для создания стильных визиток разработчика.

## Доступные языки README
[RU](README_ru.md) | [EN](README.md)

## Библиотека

### Основные компоненты

1. **Конфигурация** (`sss-core`)
   - Структуры данных для информации о разработчике

2. **Темы** (`render`, trait `Shade`)
   - Настраиваемые цветовые схемы
   - Встроенные темы:
     - Umbrella (по умолчанию)
     - Rosé Pine
     - GrooveBox
     - Dracula

3. **Макеты** (`render`, trait `Render`)
   - Система шаблонов на основе Tera
   - Поддержка HTML и CSS
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

CLI инструмент для генерации HTML с использованием тем и макетов SSS-rs.
Позволяет запускать сгенерированную страницу с автоматическим обновлением при изменении тем и макетов.

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
  -c, --config-type <TYPE>    Формат конфигурации [по умолчанию: toml]
                             [возможные значения: json, toml]
```

#### run - Запуск сервера разработки
```bash
sss_cli run [OPTIONS]

Параметры:
  -w, --watch                Отслеживание изменений конфигурации
  -s, --serve               Запуск веб-сервера
  -a, --address <ADDRESS>   Адрес веб-сервера [по умолчанию: 0.0.0.0:8081]
```

#### gen - Генерация HTML
```bash
sss_cli gen
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
# или
sss_cli run -w -s

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

## Лицензия
[Apache 2.0](LICENSE)

## Как внести свой вклад
Если вы хотите добавить новую тему или макет, создайте Issue!

# Пример
![Пример карточки](.content/umbrella.umbrella.jpeg)
