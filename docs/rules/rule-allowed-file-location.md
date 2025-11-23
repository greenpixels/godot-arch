# Rule: Allowed File Location

Ensures that specific file types are only located in designated directories.

## Purpose

This rule helps maintain a consistent project structure by restricting where certain file types can exist. For example, you might want all scenes in a `scenes/` folder and all images in an `assets/images/` folder.

## Configuration

This rule is configured via the `allowedFileLocations` map in your config file. Unlike other rules, it doesn't use `includePatterns` as the inclusion is implicitly defined via the `alloweFileLocations` configuration (see below for more information).

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.gd":
        - ./scenes/**
        - ./scripts/**
    "./**/*.{png,jpeg,jpg,gif}":
        - ./assets/images/**
```

**Format:**

- **Key:** A glob pattern matching the files to check
- **Value:** An array of glob patterns defining where those files are allowed

## Examples

### Example 1: Scene Files Only in Scenes Folder

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
```

✅ Allowed:

- `./scenes/player.tscn`
- `./scenes/enemies/goblin.tscn`

❌ Not Allowed:

- `./player.tscn`
- `./assets/player.tscn`

### Example 2: Multiple Allowed Locations

```yaml
allowedFileLocations:
    "./**/*.gd":
        - ./scenes/**
        - ./globals/**
        - ./autoload/**
```

✅ Allowed:

- `./scenes/player/player.gd`
- `./globals/game_manager.gd`
- `./autoload/save_system.gd`

❌ Not Allowed:

- `./player.gd`
- `./scripts/helper.gd`

### Example 3: Asset Organization

```yaml
allowedFileLocations:
    "./**/*.{png,jpg,jpeg,gif,webp}":
        - ./assets/images/**
    "./**/*.{mp3,wav,ogg}":
        - ./assets/audio/**
    "./**/*.{ttf,otf}":
        - ./assets/fonts/**
```

### Example 4: Resource Files

```yaml
allowedFileLocations:
    "./**/*.tres":
        - ./resources/**
    "./**/*.gdshader":
        - ./shaders/**
```

## Ignore Patterns

You can exclude specific files or folders from this rule:

```yaml
ignorePatterns:
    "rule-allowed-file-location":
        - ./test/**
        - ./examples/**
```

Or use global ignores:

```yaml
ignorePatterns:
    overall:
        - ./addons/**
```

## How to Fix Violations

When a file is in a disallowed location, you have several options:

1. **Move the file** to one of the allowed locations
2. **Add a new allowed location** to the configuration
3. **Add an ignore pattern** for that specific file or folder
4. **Remove the restriction** by deleting the entry from `allowedFileLocations`

## Common Use Cases

### Enforce Godot Project Structure

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.gd":
        - ./scenes/**
        - ./scripts/**
    "./**/*.tres":
        - ./resources/**
```

### Separate by Asset Type

```yaml
allowedFileLocations:
    "./**/*.{png,jpg,jpeg}":
        - ./assets/images/**
    "./**/*.{mp3,wav,ogg}":
        - ./assets/audio/**
    "./**/*.{glb,gltf,obj}":
        - ./assets/models/**
```

### Keep Core Files Organized

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
        - ./globals/**
    "./**/*.gd":
        - ./scenes/**
        - ./globals/**
        - ./utils/**
```

## Tips

- Start broad and tighten restrictions as your project grows
- Use the `overall` ignore pattern for third-party folders like `addons/`
- Consider your team's workflow when defining locations
- Document your chosen structure in your project's README

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
