# Glob Patterns Guide

GodotArch uses glob patterns to match files and directories. This guide explains the syntax and provides examples.

## Basic Syntax

### Wildcards

- `*` - Matches any characters except `/` (path separator)
- `**` - Matches any characters including `/` (recursive match)
- `{png,jpg}` - Matches any of the comma-separated patterns

### Path Components

- `./` - Current directory (required prefix for all paths)
- `/` - Path separator

**Important:** All paths must start with `./` to indicate they are relative to the current project directory.

## Common Patterns

### Match All Files

```yaml
- ./**
```

Matches every file in the project recursively.

### Match Specific File Type

```yaml
- ./**/*.tscn
```

Matches all `.tscn` files anywhere in the project.

### Match Multiple Extensions

```yaml
- ./**/*.{png,jpg,jpeg,gif}
```

Matches all image files with specified extensions.

### Match in Specific Folder

```yaml
- ./scenes/**
```

Matches all files within the `scenes` folder and its subdirectories.

### Match at Specific Depth

```yaml
- ./scenes/*/*.tscn
```

Matches `.tscn` files that are exactly one level deep in the `scenes` folder.

**Note:** GodotArch doesn't use the `!` negation syntax. Use `ignorePatterns` instead.

## Examples

### Example 1: Match All Script Files

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**/*.gd
```

### Example 2: Match Scenes in Specific Folders

```yaml
includePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./scenes/**/*.tscn
        - ./globals/**/*.tscn
```

### Example 3: Match All Audio Files

```yaml
allowedFileLocations:
    "./**/*.{mp3,wav,ogg}":
        - ./assets/audio/**
```

### Example 4: Ignore Build and Export Folders

```yaml
ignorePatterns:
    overall:
        - ./export/**
        - ./build/**
        - ./.godot/**
```

### Example 5: Match Hidden Files

```yaml
ignorePatterns:
    overall:
        - ./**/.{*}
```

Matches all hidden files (starting with `.`) like `.gitignore`, `.godot`, etc.


## Pattern Precedence

1. Files are first matched against `includePatterns`
2. Then checked against `ignorePatterns.overall`
3. Finally checked against rule-specific `ignorePatterns`

If a file matches an ignore pattern, it will not be checked, even if it matches an include pattern.
