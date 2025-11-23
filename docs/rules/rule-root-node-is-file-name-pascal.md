# Rule: Root Node Is File Name Pascal

Ensures that a scene's root node name matches the scene file name (converted to PascalCase).

## Purpose

This rule maintains consistency between scene file names and their root nodes, making it easier to understand what each scene represents.

## How It Works

The rule converts the scene file name from `snake_case` to `PascalCase` and checks if the root node matches.

**Examples:**

| File Name | Expected Root Node |
|-----------|-------------------|
| `player.tscn` | `Player` |
| `main_menu.tscn` | `MainMenu` |
| `enemy_goblin.tscn` | `EnemyGoblin` |
| `level_01.tscn` | `Level01` |

## Configuration

### Check All Scenes

```yaml
includePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./**/*.tscn
```

### Check Specific Folders

```yaml
includePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./scenes/**/*.tscn
        - ./levels/**/*.tscn
```

## Ignore Patterns

```yaml
ignorePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./test/**
        - ./addons/**
```

## How to Fix Violations

### Example 1: File Name Matches Root Node

**File:** `player.tscn`  
**Current Root Node:** `Node2D`  
**Fix:** Rename root node to `Player`

### Example 2: Root Node Matches File Name

**File:** `main_scene.tscn`  
**Current Root Node:** `MainMenu`  
**Fix:** Rename file to `main_menu.tscn`

### Example 3: Both Need Adjustment

**File:** `player_character.tscn`  
**Current Root Node:** `Player`  
**Options:**

- Rename file to `player.tscn`, OR
- Rename root node to `PlayerCharacter`

## Common Examples

### Player Scene

```
File: player.tscn
Root Node: Player (CharacterBody2D)
```

### Main Menu

```
File: main_menu.tscn
Root Node: MainMenu (Control)
```

### Enemy Variants

```
File: enemy_goblin.tscn
Root Node: EnemyGoblin (CharacterBody2D)

File: enemy_skeleton.tscn
Root Node: EnemySkeleton (CharacterBody2D)
```

### Level Scenes

```
File: level_01.tscn
Root Node: Level01 (Node2D)

File: level_forest.tscn
Root Node: LevelForest (Node2D)
```

## Benefits

1. **Clarity:** Instant recognition of scene purpose
2. **Consistency:** Predictable naming across project
3. **Navigation:** Easy to find corresponding files
4. **Refactoring:** Renaming one updates the other

## Common Patterns

### With Parent Has Same Name Rule

```
scenes/
├── player/
│   ├── player.tscn          # Root: Player
│   └── player.gd
└── enemy_goblin/
    ├── enemy_goblin.tscn    # Root: EnemyGoblin
    └── enemy_goblin.gd
```

### With Root Node Script In Same Folder

```
player/
├── player.tscn              # Root: Player
└── player.gd                # Attached to Player node
```

## Special Cases

### Numbered Scenes

```yaml
# These conversions happen automatically:
level_01.tscn    → Level01
enemy_02.tscn    → Enemy02
room_3a.tscn     → Room3a
```

### Underscore Handling

```yaml
# Underscores are removed and next letter capitalized:
my_awesome_scene.tscn → MyAwesomeScene
player_controller.tscn → PlayerController
```

## Tips

- Use with [Filename Snake Case](./rule-filename-snake-case.md) for consistency
- Apply to all `.tscn` files for best results
- Great for onboarding new team members

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
