# Available Template Fields

## Colors
- `primary`: String - Primary text color
- `secondary`: String - Background color
- `thirdly`: String - Accent color
- `border`: String - Muted color for borders, text, and dividers

## User Data
- `name`: String - Username
- `current_nickname`: Object
  - `word`: String - Current nickname
  - `pronounce`: String - Nickname pronunciation
- `previous_nicknames`: Array[Object]
  - `word`: String - Previous nickname
  - `pronounce`: String - Previous nickname pronunciation

## Professional Info
- `specifications`: Array[String] - List of specializations/roles
- `about`: String - Description text

## Repository Links
- `repos`: Array[Object]
  - `name`: String - Repository name
  - `link`: Object
    - `provider`: Enum(Github|LinkedIn|Telegram)
    - `url`: String - Repository URL

## Social Links
- `socials`: Array[Object]
  - `provider`: Enum(Github|LinkedIn|Telegram)
  - `url`: String - Social network URL

## Skills
- `skills`: Array[Object]
  - `skill`: String - Skill name
  - `main`: Boolean - Is it a main skill
  - `since`: Object
    - `start`: Number - Start year
    - `end`: Number - End year (0 = "present")
  - `repo_link`: Object
    - `provider`: Enum(Github|LinkedIn|Telegram)
    - `url`: String - Related repository URL
  - `projects`: Array[Object]
    - `name`: String - Project name
    - `link`: Object
      - `provider`: Enum(Github|LinkedIn|Telegram)
      - `url`: String - Project URL

## Helper Functions
- `get_svg(provider: Enum)`: Function - Returns SVG icon for specified provider

# Tera Template Constructions
- Example based on [umbrella layout](C:\Users\towinok\Documents\SSS-rs\sss-lib\sss-std\src\layouts\templates\umbrella\card.html.tera)
- [Official documentation](https://keats.github.io/tera/)

## 1. Value Insertion
```tera
# Simple variable insertion
{{ secondary }}

# Nested value insertion
{{ current_nickname.word }}
{{ current_nickname.pronounce }}

# Insertion in CSS classes
class="text-[{{ primary }}]"
class="bg-[{{ secondary }}]"
```

## 2. Loops (For)
```tera
# Simple array loop
{% for spec in specifications %}
    <span>{{ spec }}</span>
{% endfor %}

# Loop through array of objects
{% for repo in repos %}
    <a href="{{ repo.link.link }}">
        {{ repo.name }}
    </a>
{% endfor %}
```

## 3. Conditional Statements (If)
```tera
# Simple condition
{% if skill.main %}
    {% set opacity = 100 %}
{% else %}
    {% set opacity = 80 %}
{% endif %}

# Inline condition
{{ skill.since.end != 0 ? skill.since.end : "today" }}

# Condition with multiple checks
{% if skill.main and skill.since.end == 0 %}
    // content
{% endif %}
```

## 4. Variable Assignment
```tera
{% set opacity = 100 %}
```

## 5. Function Calls with Filters
```tera
# Function call with parameter and safe filter
{{ get_svg(provider=repo.link.provider) | safe }}
```

## 6. Comments
```tera
{# This is a comment in Tera #}
```

## 7. Nested Conditions Inside Loops
```tera
{% for skill in skills %}
    {% if skill.main %}
        {% set opacity = 100 %}
    {% else %}
        {% set opacity = 80 %}
    {% endif %}
    <div class="opacity-{{ opacity }}">
        {{ skill.name }}
    </div>
{% endfor %}
```

## 8. Accessing Nested Structures
```tera
# Two-level nesting
{{ skill.since.start }}

# Condition with nested field
{% if skill.since.end != 0 %}
    {{ skill.since.end }}
{% else %}
    today
{% endif %}
```
