# Rule: Root Node Script In Same Folder

Ensures that the script attached to a scene's root node is located in the same folder as the scene file.

## Purpose

This rule keeps related files together, making your project easier to navigate and maintaining a clear connection between scenes and their scripts.

## Structure Example

✅ **Valid Structure:**

```
player/
├── player.tscn          # Scene file
└── player.gd            # Root node script
```

❌ **Invalid Structure:**

```
scenes/
├── player.tscn          # Scene file
scripts/
└── player.gd            # Root node script (wrong location!)
```

## Configuration

### Check All Scenes

```yaml
includePatterns:
    "rule-root-node-script-in-same-folder":
        - ./**/*.tscn
```

### Check Specific Folders

```yaml
includePatterns:
    "rule-root-node-script-in-same-folder":
        - ./scenes/**/*.tscn
        - ./levels/**/*.tscn
```

## Ignore Patterns

```yaml
ignorePatterns:
    "rule-root-node-script-in-same-folder":
        - ./globals/**     # Globals may reference external scripts
        - ./test/**
```

## How to Fix Violations

### Option 1: Move the Script

If you have:

```
scenes/player/player.tscn
scripts/player.gd
```

Move to:

```
scenes/player/player.tscn
scenes/player/player.gd
```

### Option 2: Update the Scene

If the script can't be moved, update the scene to reference a script that IS in the same folder, or add an exception.

### Option 3: Ignore the Scene

```yaml
ignorePatterns:
    "rule-root-node-script-in-same-folder":
        - ./scenes/special_case.tscn
```

## Common Patterns

### Standard Entity Structure

```
scenes/
├── player/
│   ├── player.tscn
│   ├── player.gd            # ✅ Same folder
│   └── player_sprite.png
└── enemy/
    ├── enemy.tscn
    ├── enemy.gd             # ✅ Same folder
    └── enemy_animation.tres
```

### UI Components

```
ui/
├── main_menu/
│   ├── main_menu.tscn
│   ├── main_menu.gd         # ✅ Same folder
│   └── button_theme.tres
└── inventory/
    ├── inventory.tscn
    ├── inventory.gd         # ✅ Same folder
    └── slot.tscn
```

### Level Scenes

```
levels/
├── level_01/
│   ├── level_01.tscn
│   ├── level_01.gd          # ✅ Same folder
│   └── level_01_tilemap.tres
└── level_02/
    ├── level_02.tscn
    ├── level_02.gd          # ✅ Same folder
    └── background.png
```

## What the Rule Checks

1. **Scene has a root node** with a script attached
2. **Script is an external resource** (not built-in)
3. **Script path** matches the scene's directory

The rule automatically:

- ✅ Passes if no script is attached
- ✅ Passes if the script is built-in
- ❌ Fails if external script is in a different folder

## Built-in Scripts

This rule only applies to external scripts. Built-in scripts (embedded in the `.tscn` file) automatically pass.

## Benefits

1. **Organization:** Related files stay together
2. **Navigation:** Easy to find a scene's script
3. **Refactoring:** Moving a scene moves its script
4. **Clarity:** Clear ownership of scripts

## Common Exceptions

You may want to exclude:

```yaml
ignorePatterns:
    "rule-root-node-script-in-same-folder":
        - ./globals/**      # Global/singleton scripts
        - ./autoload/**     # Autoload scripts
        - ./addons/**       # Third-party addons
```

## Tips

- Apply this rule to all `.tscn` files
- Use with [Parent Has Same Name](./rule-parent-has-same-name.md)
- Create a consistent folder-per-scene structure
- Great for team projects

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
