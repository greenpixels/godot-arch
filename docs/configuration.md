# Configuration Reference

This document covers all available configuration options in `godot-arch.config.yaml`.

## Top-Level Options

### `shouldPrintSuccess`

**Type:** `boolean`  
**Default:** `false`

Controls whether successful rule validations are printed to the console. When `false`, only failures and warnings are shown.

```yaml
shouldPrintSuccess: true
```

**Use Case:** Helpful during initial setup to see which files are passing validation.

### `waitForInputBeforeClose`

**Type:** `boolean`  
**Default:** `false`

Makes the program wait for user input before exiting. Useful for Windows users who double-click the executable.

```yaml
waitForInputBeforeClose: true
```

### `maxNodeDepth`

**Type:** `number`  
**Default:** `4`

Sets the maximum allowed depth for nodes in scene trees. Used by the [Node Depth Fits Max Depth](./rules/rule-node-depth-fits-max-depth.md) rule.

```yaml
maxNodeDepth: 3
```

### `allowScreamingSnakeCaseInNodeNames`

**Type:** `boolean`  
**Default:** `false`

When `true`, allows `SCREAMING_SNAKE_CASE` node names in addition to `PascalCase`. Used by the [Scene Nodes Pascal Case](./rules/rule-scene-nodes-pascal-case.md) rule.

```yaml
allowScreamingSnakeCaseInNodeNames: true
```

**Note:** This is useful for constant-like nodes or when working with native Godot nodes that use this convention.

### `nodeNamePascalCaseExceptions`

**Type:** `array of maps`  
**Default:** `[]`

Defines remapping rules for node names that don't conform to PascalCase. Each entry maps a non-conforming pattern to its acceptable form. Used by the [Scene Nodes Pascal Case](./rules/rule-scene-nodes-pascal-case.md) rule.

```yaml
nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - CPUParticles2D: CpuParticles2D
    - HTTPRequest: HttpRequest
```

**Example:** A node named `GPUParticles2D` will be treated as valid `GpuParticles2D`.

### `allowedFileLocations`

**Type:** `map of glob patterns to arrays of glob patterns`  
**Default:** `{}`

Defines where specific file types are allowed to exist. See [Allowed File Location Rule](./rules/rule-allowed-file-location.md) for details.

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.{png,jpeg}":
        - ./assets/images/**
```

## Include and Ignore Patterns

### `includePatterns`

Defines which files each rule should check. Each rule (except `rule-allowed-file-location`) can have its own include patterns.

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**
    "rule-parent-has-same-name":
        - ./**/*.tscn
        - ./**/*.gd
    "rule-scene-nodes-pascal-case":
        - ./**/*.tscn
    "rule-root-node-is-file-name-pascal":
        - ./**/*.tscn
    "rule-root-node-script-in-same-folder":
        - ./**/*.tscn
    "rule-node-depth-fits-max-depth":
        - ./**/*.tscn
```

**Note:** See [Glob Patterns Guide](./glob-patterns.md) for pattern syntax.

### `ignorePatterns`

Defines which files to exclude from checking. You can set global ignores or per-rule ignores.

#### Global Ignores

The `overall` pattern applies to all rules:

```yaml
ignorePatterns:
    overall:
        - ./addons/**
        - ./export/**
        - ./.godot/**
        - ./**/.{*}  # Hidden files
```

#### Per-Rule Ignores

Each rule can have specific ignore patterns:

```yaml
ignorePatterns:
    "rule-filename-snake-case":
        - ./legacy/**
    "rule-parent-has-same-name":
        - ./globals/**
    # ... other rules
```

**Available Rule Names:**

- `rule-allowed-file-location`
- `rule-filename-snake-case`
- `rule-parent-has-same-name`
- `rule-scene-nodes-pascal-case`
- `rule-root-node-is-file-name-pascal`
- `rule-root-node-script-in-same-folder`
- `rule-node-depth-fits-max-depth`

## Complete Example

```yaml
shouldPrintSuccess: false
waitForInputBeforeClose: false
maxNodeDepth: 4
allowScreamingSnakeCaseInNodeNames: false

nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - CPU: Cpu

includePatterns:
    "rule-filename-snake-case":
        - ./**
    "rule-parent-has-same-name":
        - ./**/*.tscn
        - ./**/*.gd

ignorePatterns:
    overall:
        - ./addons/**
        - ./export/**
    "rule-filename-snake-case":
        - ./project.godot

allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.gd":
        - ./scenes/**
        - ./scripts/**
```
