# Rule: Node Depth Fits Max Depth

Ensures that nodes in scene trees don't exceed a specified maximum nesting depth.

## Purpose

This rule prevents overly complex scene hierarchies that can be hard to navigate, maintain, and may indicate a need for refactoring into separate scenes.

## How It Works

The rule counts how many levels deep a node is nested in the scene tree:

```
Root (depth 0)
└── Child (depth 1)
    └── GrandChild (depth 2)
        └── GreatGrandChild (depth 3)
```

## Configuration

### Set Maximum Depth

```yaml
maxNodeDepth: 3
```

**Default:** `4`

### Enable the Rule

```yaml
includePatterns:
    "rule-node-depth-fits-max-depth":
        - ./**/*.tscn
```

### Complete Example

```yaml
maxNodeDepth: 3

includePatterns:
    "rule-node-depth-fits-max-depth":
        - ./**/*.tscn
```

## Ignore Patterns

```yaml
ignorePatterns:
    "rule-node-depth-fits-max-depth":
        - ./ui/**          # UI might need deeper hierarchies
        - ./test/**
```

## Depth Calculation Examples

### Example 1: Simple Scene (Max Depth 2)

```
Player (depth 0) ✅
├── Sprite (depth 1) ✅
└── CollisionShape (depth 1) ✅
```

**All nodes:** ✅ Pass with `maxNodeDepth: 2`

### Example 2: Deeper Scene (Max Depth 2)

```
Player (depth 0) ✅
└── Body (depth 1) ✅
    └── Sprite (depth 2) ✅
        └── AnimationPlayer (depth 3) ❌
```

**AnimationPlayer:** ❌ Fails with `maxNodeDepth: 2`

### Example 3: UI Scene (Max Depth 3)

```
MainMenu (depth 0) ✅
└── MarginContainer (depth 1) ✅
    └── VBoxContainer (depth 2) ✅
        └── PlayButton (depth 3) ✅
            └── Icon (depth 4) ❌
```

**Icon:** ❌ Fails with `maxNodeDepth: 3`

## How to Fix Violations

### Option 1: Refactor Into Subscenes

**Before:**

```
Player (depth 0)
└── Body (depth 1)
    └── VisualGroup (depth 2)
        └── Sprite (depth 3)
            └── AnimationPlayer (depth 4) ❌
```

**After:**

Create `player_visuals.tscn`:

```
VisualGroup (depth 0)
├── Sprite (depth 1)
└── AnimationPlayer (depth 2) ✅
```

Reference in `player.tscn`:

```
Player (depth 0)
└── Body (depth 1)
    └── VisualGroup (depth 2) [instanced scene] ✅
```

### Option 2: Flatten Hierarchy

**Before:**

```
Enemy (depth 0)
└── Container (depth 1)
    └── Wrapper (depth 2)
        └── Sprite (depth 3)
            └── Animation (depth 4) ❌
```

**After:**

```
Enemy (depth 0)
├── Sprite (depth 1)
└── Animation (depth 2) ✅
```

### Option 3: Increase Max Depth

If your project legitimately needs deeper hierarchies:

```yaml
maxNodeDepth: 5
```

### Option 4: Ignore Specific Scenes

```yaml
ignorePatterns:
    "rule-node-depth-fits-max-depth":
        - ./ui/complex_dialog.tscn
```

## Common Patterns

### Good: Flat Structure with Subscenes

```
Player (depth 0)
├── Sprite (depth 1)
├── CollisionShape (depth 1)
├── HealthBar (depth 1) [instanced subscene]
└── Weapon (depth 1) [instanced subscene]
```

### Bad: Deep Nested Structure

```
Player (depth 0)
└── Body (depth 1)
    └── Visual (depth 2)
        └── Mesh (depth 3)
            └── Material (depth 4)
                └── Texture (depth 5) ❌
```

**Refactor to:**

```
Player (depth 0)
├── Mesh (depth 1)
└── CollisionShape (depth 1)
```

## Benefits

1. **Maintainability:** Easier to understand scene structure
2. **Performance:** Flatter hierarchies can be more efficient
3. **Reusability:** Encourages creating reusable subscenes
4. **Collaboration:** Simpler for team members to navigate

## When to Ignore

You might want to ignore this rule for:

- **Complex UI layouts** with many container nodes
- **Generated scenes** from external tools
- **Third-party assets** in the `addons/` folder
- **Test scenes** for debugging

```yaml
ignorePatterns:
    "rule-node-depth-fits-max-depth":
        - ./ui/**
        - ./addons/**
        - ./test/**
```

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
