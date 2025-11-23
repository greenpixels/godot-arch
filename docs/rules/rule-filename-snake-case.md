# Rule: Filename Snake Case

Ensures that file names use `snake_case` naming convention.

## Purpose

This rule enforces consistent file naming across your project by requiring all file names to be in `snake_case` format (lowercase with underscores).

## What is Snake Case?

`snake_case` uses lowercase letters with underscores separating words:

✅ **Valid:**

- `player_controller.gd`
- `main_menu.tscn`
- `enemy_01.png`
- `level_background_music.mp3`

❌ **Invalid:**

- `PlayerController.gd` (PascalCase)
- `mainMenu.tscn` (camelCase)
- `Enemy-01.png` (kebab-case)
- `PLAYER.gd` (SCREAMING_SNAKE_CASE)

## Configuration

Add files to check in your `includePatterns`:

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**
```

### Check Specific File Types

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**/*.gd
        - ./**/*.tscn
        - ./**/*.tres
```

### Check Everything

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**
```

## Ignore Patterns

Exclude files or folders from this rule:

```yaml
ignorePatterns:
    "rule-filename-snake-case":
        - ./addons/**
        - ./project.godot
```

## How to Fix Violations

1. **Rename the file** to use snake_case
   - `PlayerController.gd` → `player_controller.gd`
   - `MainMenu.tscn` → `main_menu.tscn`

2. **Add an exception** if the file must keep its name

   ```yaml
   ignorePatterns:
       "rule-filename-snake-case":
           - ./specific_file.gd
   ```

3. **Disable for entire folders** (e.g., third-party code)

   ```yaml
   ignorePatterns:
       "rule-filename-snake-case":
           - ./addons/**
   ```

## Common Exceptions

You may want to ignore:

```yaml
ignorePatterns:
    "rule-filename-snake-case":
        - ./project.godot  # Godot project file
        - ./addons/**      # Third-party addons
        - ./export/**      # Export templates
        - ./.godot/**      # Godot cache
```

## Why Snake Case?

- **Consistency:** One naming style across all files
- **Readability:** Easy to distinguish between words
- **Cross-platform:** No case-sensitivity issues
- **Convention:** Matches common Python/GDScript style

## Tips

- Apply this rule project-wide for maximum consistency
- Set it up early in your project to avoid mass renaming later
- Use with [Parent Has Same Name](./rule-parent-has-same-name.md) for a cohesive structure
- Remember: file extensions (`.gd`, `.tscn`) don't need to match the rule

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
