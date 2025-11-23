# Rule: Parent Has Same Name

Ensures that files are placed inside a folder with the same name as the file.

## Purpose

This rule promotes a well-organized project structure where each significant file gets its own folder, making it easier to find related assets and preventing clutter.

## Structure Example

✅ **Valid Structure:**

```
scenes/
├── player/
│   ├── player.tscn
│   └── player.gd
├── enemy/
│   ├── enemy.tscn
│   └── enemy.gd
└── level_01/
    ├── level_01.tscn
    └── level_01.gd
```

❌ **Invalid Structure:**

```
scenes/
├── player.tscn
├── player.gd
├── enemy.tscn
└── enemy.gd
```

## Configuration

### Check Scene and Script Files

```yaml
includePatterns:
    "rule-parent-has-same-name":
        - ./**/*.tscn
        - ./**/*.gd
```

### Check All Files

```yaml
includePatterns:
    "rule-parent-has-same-name":
        - ./**
```

### Check Only Scenes

```yaml
includePatterns:
    "rule-parent-has-same-name":
        - ./**/*.tscn
```

## Ignore Patterns

```yaml
ignorePatterns:
    "rule-parent-has-same-name":
        - ./globals/**  # Global scripts don't need folders
        - ./autoload/**
```

## How to Fix Violations

### Option 1: Create a Matching Folder

If you have:

```
scenes/player.tscn
```

Create:

```
scenes/player/player.tscn
```

### Option 2: Rename the File

If you have:

```
scenes/entities/player.tscn
```

Rename to:

```
scenes/entities/entities.tscn
```

### Option 3: Ignore the File

```yaml
ignorePatterns:
    "rule-parent-has-same-name":
        - ./scenes/main.tscn
```

## Common Use Cases

### Game Entities

```
scenes/
├── player/
│   ├── player.tscn
│   ├── player.gd
│   ├── player_animation.tres
│   └── sprites/
│       └── player_idle.png
├── enemy_goblin/
│   ├── enemy_goblin.tscn
│   ├── enemy_goblin.gd
│   └── goblin_sprite.png
```

### UI Components

```
ui/
├── main_menu/
│   ├── main_menu.tscn
│   ├── main_menu.gd
│   └── main_menu_theme.tres
├── inventory/
│   ├── inventory.tscn
│   ├── inventory.gd
│   └── slot.tscn
```

### Levels

```
levels/
├── level_01/
│   ├── level_01.tscn
│   ├── level_01.gd
│   └── level_01_tilemap.tres
├── level_02/
│   ├── level_02.tscn
│   └── level_02.gd
```

## Benefits

1. **Organization:** Related files are grouped together
2. **Scalability:** Easy to add more assets per entity
3. **Navigation:** Quick to find all files for a feature
4. **Clarity:** Structure reflects the game's architecture

## Common Exceptions

You may want to exclude:

```yaml
ignorePatterns:
    "rule-parent-has-same-name":
        - ./globals/**      # Singletons/autoload scripts
        - ./utils/**        # Utility scripts
        - ./resources/**    # Shared resources
```

## Tips

- Works great with [Filename Snake Case](./rule-filename-snake-case.md)
- Start using this rule early in development
- Create a project template with this structure

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
