# Rule: Allowed Custom Resource Location

Ensures that custom resource files are located in designated directories.

## Purpose

This rule helps maintain a consistent project structure by restricting where custom resource files can exist. For example, you might want all `Hat` resources to be in the `resources/hats/all` folder.

## Configuration

This rule is configured via the `allowedCustomResourceLocations` map in your config file. Unlike other rules, it doesn't use `includePatterns` as the inclusion is implicitly defined via the `allowedCustomResourceLocations` configuration (see below for more information).

Additionally, the `failUnmatchedCustomResources` flag can be used to enforce stricter validation, as it fails if a custom resource has not been included in `allowedCustomResourceLocations`.

```yaml
allowedCustomResourceLocations:
    "File":
        - ./resources/**
        - ./assets/resources/**
failUnmatchedCustomResources: true
```

**Format:**

- **Key:** The name of the custom resource type (e.g., `File`)
- **Value:** An array of glob patterns defining where those files are allowed
- **failUnmatchedCustomResources:** A boolean flag to determine whether unmatched custom resources should cause validation to fail.

## Examples

### Example 1: Custom Resource Files in Resources Folder

```yaml
allowedCustomResourceLocations:
    "File":
        - ./resources/**
```

✅ Allowed:

- `./resources/config.tres`
- `./resources/subfolder/data.tres`

❌ Not Allowed:

- `./config.tres`
- `./assets/config.tres`

### Example 2: Multiple Allowed Locations

```yaml
allowedCustomResourceLocations:
    "File":
        - ./resources/**
        - ./assets/resources/**
```

✅ Allowed:

- `./resources/config.tres`
- `./assets/resources/config.tres`

❌ Not Allowed:

- `./config.tres`
- `./assets/config.tres`

## Ignore Patterns

You can exclude specific files or folders from this rule:

```yaml
ignorePatterns:
    "rule-allowed-custom-resource-location":
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
4. **Remove the restriction** by deleting the entry from `allowedCustomResourceLocations`

## Common Use Cases

### Enforce Custom Resource Organization

```yaml
allowedCustomResourceLocations:
    "File":
        - ./resources/**
    "Shader":
        - ./shaders/**
```

### Separate by Resource Type

```yaml
allowedCustomResourceLocations:
    "File":
        - ./resources/**
    "Shader":
        - ./shaders/**
    "Material":
        - ./materials/**
```

## Related Documentation

- [Configuration Reference](../configuration.md)
- [Glob Patterns Guide](../glob-patterns.md)
