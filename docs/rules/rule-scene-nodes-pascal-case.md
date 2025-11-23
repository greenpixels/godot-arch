# Rule: Scene Nodes Pascal Case

Ensures that node names in scene trees use `PascalCase` naming convention.

## Purpose

This rule enforces consistent node naming within your Godot scenes, making scene trees more readable and professional.

## What is PascalCase?

`PascalCase` uses capitalized words with no separators:

✅ **Valid:**

- `PlayerController`
- `MainCamera`
- `EnemySpawner`
- `UIContainer`

❌ **Invalid:**

- `player_controller` (snake_case)
- `mainCamera` (camelCase)
- `enemy-spawner` (kebab-case)
- `ENEMY_SPAWNER` (SCREAMING_SNAKE_CASE)

## Configuration

### Check All Scenes

```yaml
includePatterns:
    "rule-scene-nodes-pascal-case":
        - ./**/*.tscn
```

### Enable SCREAMING_SNAKE_CASE

If you want to allow `SCREAMING_SNAKE_CASE` in addition to `PascalCase`:

```yaml
allowScreamingSnakeCaseInNodeNames: true
```

This is useful for constant-like nodes or configuration nodes.

### Handle Abbreviations and Special Cases

Some node names (especially native Godot nodes) don't strictly follow PascalCase:

```yaml
nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - CPU: Cpu
    - HTTP: Http
    - CPUParticles2D: CpuParticles2D
    - GPUParticles2D: GpuParticles2D
    - HTTPRequest: HttpRequest
```

**How it works:** A node named `GPUParticles2D` will be treated as valid `GpuParticles2D`.

## Ignore Patterns

```yaml
ignorePatterns:
    "rule-scene-nodes-pascal-case":
        - ./test/**
        - ./examples/**
```

## How to Fix Violations

1. **Rename the node** in the Godot editor
   - `player_node` → `PlayerNode`
   - `enemy_1` → `Enemy1`

2. **Add an exception** for special cases

   ```yaml
   nodeNamePascalCaseExceptions:
       - API: Api
       - ID: Id
   ```

3. **Ignore specific scenes**

   ```yaml
   ignorePatterns:
       "rule-scene-nodes-pascal-case":
           - ./legacy/**
   ```

## Common Examples

### Player Scene

```
Player (Node2D)
├── Sprite
├── CollisionShape
├── AnimationPlayer
├── HealthBar
│   ├── ProgressBar
│   └── Label
└── InputHandler
```

### UI Scene

```
MainMenu (Control)
├── Background
├── Logo
├── ButtonContainer
│   ├── PlayButton
│   ├── OptionsButton
│   └── QuitButton
└── VersionLabel
```

### Level Scene

```
Level01 (Node2D)
├── TileMap
├── PlayerSpawnPoint
├── EnemyContainer
│   ├── Goblin
│   └── Skeleton
└── ItemContainer
    ├── HealthPotion
    └── Coin
```

## Native Godot Nodes

Most Godot nodes already follow PascalCase:

- `Node2D`
- `CharacterBody2D`
- `AnimationPlayer`
- `CollisionShape2D`

## Special Cases and Exceptions

### Abbreviations

Common abbreviations in Godot:

```yaml
nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - CPU: Cpu
    - UI: Ui
    - FPS: Fps
    - HUD: Hud
    - AI: Ai
    - HP: Hp
    - MP: Mp
```

### Numbers

Numbers are allowed in PascalCase:

- ✅ `Enemy1`
- ✅ `Level2Boss`
- ✅ `Player2Controller`

### Allow SCREAMING_SNAKE_CASE

For constants or config nodes:

```yaml
allowScreamingSnakeCaseInNodeNames: true
```

Then both are valid:

- ✅ `PlayerController`
- ✅ `MAX_HEALTH_CONFIG`

## Benefits

- **Consistency:** Uniform naming across all scenes
- **Readability:** Clear node hierarchy
- **Professional:** Matches Godot conventions
- **Searchability:** Easy to find nodes in large scenes

## Tips

- Enable this rule project-wide
- Add exceptions for native Godot nodes as needed
- Use descriptive names: `PlayerHealthBar` not `ProgressBar`

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
