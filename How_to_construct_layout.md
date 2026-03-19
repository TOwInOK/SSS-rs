# Available Template Fields

## Colors
- `text`: String - Primary text color
- `background`: String - Background color
- `accent`: String - Accent color
- `border`: String - Muted color for borders, text, and dividers

## User Data
- `name`: String - Username
- `current_nickname`: Object
  - `word`: String - Current nickname
  - `pronounce`: String - Nickname pronunciation
- `prevision_nicknames`: Array[Object]
  - `word`: String - Previous nickname
  - `pronounce`: String - Previous nickname pronunciation

## Professional Info
- `specifications`: Array[String] - List of specializations/roles
- `about`: String - Description text

## Repository Links
- `repos`: Array[Object]
  - `name`: String - Repository name
  - `link`: Object
    - `icon`: Tabler - Icon identifier (Tabler enum)
    - `link`: String - Repository URL

## Social Links
- `socials`: Array[Object]
  - `icon`: Tabler - Icon identifier (Tabler enum)
  - `link`: String - Social network URL

## Skills
- `skills`: Array[Object]
  - `skill`: String - Skill name
  - `main`: Boolean - Is it a main skill
  - `since`: Object
    - `start`: Number - Start year
    - `end`: Number - End year (0 = "present")
  - `repo_link`: Object
    - `icon`: Tabler - Icon identifier (Tabler enum)
    - `link`: String - Related repository URL
  - `projects`: Array[Object]
    - `name`: String - Project name
    - `link`: Object
      - `icon`: Tabler - Icon identifier (Tabler enum)
      - `link`: String - Project URL

## Helper Functions
- `get_svg(icon: Tabler)`: Function - Returns SVG icon for specified icon identifier
- `get_icon_name(icon: Tabler)`: Function - Returns the name of the icon

# Tera Template Constructions
- Example based on [umbrella layout](sss-std/src/layouts/html_tera/templates/umbrella/card.html.tera)
- [Official documentation](https://keats.github.io/tera/)

## 1. Value Insertion
```tera
# Simple variable insertion
{{ background }}

# Nested value insertion
{{ current_nickname.word }}
{{ current_nickname.pronounce }}

# Insertion in CSS classes
class="text-[{{ text }}]"
class="bg-[{{ background }}]"
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
{{ get_svg(icon=repo.link.icon) | safe }}

# Get icon name
{{ get_icon_name(icon=social.icon) }}
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
        {{ skill.skill }}
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

# Accessing Link structure
{{ repo.link.link }}     # URL
{{ repo.link.icon }}      # Icon identifier

# Using helper functions with nested fields
{{ get_svg(icon=repo.link.icon) | safe }}
{{ get_icon_name(icon=skill.repo_link.icon) }}
```

## 9. Real Example from Umbrella Template
```tera
{# Names section #}
<div class="items-center text-[{{ text }}] grid grid-cols-2">
    <div class="text-lg md:text-xl font-bold flex justify-evenly">
        {{ name }}
    </div>
    <div class="flex items-center justify-evenly w-full text-2xl md:text-3xl">
        <span>[</span>
        <div class="flex flex-col items-center text-sm md:text-md truncate">
            <span>{{ current_nickname.word }}</span>
            <span class="text-[{{ accent }}]/80">{{ current_nickname.pronounce }}</span>
        </div>
        <span>]</span>
    </div>
</div>

{# Section with specifications and about info #}
<div class="grid grid-cols-2 gap-2 md:gap-4">
    <div class="grid gap-2">
        {% for spec in specifications %}
        <div class="text-[{{ text }}] flex items-center justify-between text-xl md:text-2xl gap-4">
            <span class="w-[15px]">[</span>
            <span class="text-xs md:text-sm text-center flex-1">{{ spec }}</span>
            <span class="w-[15px]">]</span>
        </div>
        {% endfor %}
    </div>
    <div class="text-[{{ accent }}] text-right text-sm md:text-base text-balance max-h-[200px] truncate">
        {{ about }}
    </div>
</div>

{# Section with repos #}
<div class="flex justify-evenly w-full text-[{{ text }}]">
    {% for repo in repos %}
    <a href="{{ repo.link.link }}" class="flex items-center justify-evenly w-full text-xl md:text-2xl transition duration-300 ease-in-out group">
        <span class="group-hover:scale-92 transition-transform duration-300">[</span>
        <div class="h-[48px] flex-col justify-evenly items-center flex text-[{{ accent }}]">
            <div class="group-hover:scale-108 transition-transform duration-300">
                {{ get_svg(icon=repo.link.icon) | safe }}
            </div>
            <div class="flex text-[{{ text }}] text-xs md:text-sm">
                {{ repo.name }}
            </div>
        </div>
        <span class="group-hover:scale-92 transition-transform duration-300">]</span>
    </a>
    {% endfor %}
</div>

{# Social section #}
<div class="flex justify-evenly w-full text-[{{ text }}] gap-4">
    {% for social in socials %}
    <a href="{{ social.link }}" class="group flex items-center justify-evenly w-full text-xl md:text-2xl transition duration-300 ease-in-out">
        <span class="group-hover:scale-92 transition-transform duration-300">[</span>
        <div class="h-[48px] flex-col justify-evenly items-center flex text-[{{ accent }}]">
            <div class="group-hover:scale-108 transition-transform duration-300">
                {{ get_svg(icon=social.icon) | safe }}
            </div>
            <div class="flex text-[{{ text }}] text-xs md:text-sm">
                {{ get_icon_name(icon=social.icon) }}
            </div>
        </div>
        <span class="group-hover:scale-92 transition-transform duration-300">]</span>
    </a>
    {% endfor %}
</div>

{# Skill section #}
<div class="flex justify-evenly gap-4">
    {% for skill in skills %}
        {% if skill.main %}
            {% set opacity = 100 %}
        {% else %}
            {% set opacity = 80 %}
        {% endif %}
    <a href="{{ skill.repo_link.link }}" class="text-center opacity-{{ opacity }} transition duration-300 ease-in-out group">
        <div class="text-xl md:text-2xl text-[{{ text }}] group-hover:scale-102 transition-transform duration-300">
            {{ skill.skill }}
        </div>
        <div class="flex justify-center text-xs md:text-sm text-[{{ accent }}]">
            {{ skill.since.start }} -> {% if skill.since.end != 0 %}{{ skill.since.end }}{% else %}today{% endif %}
        </div>
        <div class="group-hover:scale-105 group-hover:text-[{{ text }}] transition-all duration-300 flex justify-center text-xs md:text-sm text-[{{ border }}] hover:scale-105 transition duration-300 ease-in-out">
            {{ get_icon_name(icon=skill.repo_link.icon) }}
        </div>
    </a>
    {% endfor %}
</div>
```
