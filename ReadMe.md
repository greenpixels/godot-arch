![GodotArch](assets/images/godot-arch-logo.png)

> 🚧👷‍♂️ **Please Note:** GodotArch is still in early development. Please report any bugs and issues you find.

An extremely fast project structure linter four your Godot projects. GodotArch enforces consistent file organization and naming conventions across your Godot projects, making them more maintainable and easier to navigate. 

It can check scenes, scripts, nodes, assets, and more. It is also extremely configurable via its `godot-arch.config.yaml`. Check out the **Rules**-Section for more information.

## How To Use

In order to use GodotArch you need to have its executable and configuration inside your project root.
### 1. Requirements
- A Godot project developed with `Godot 4.4` or higher 

### 2. Installation & Setup
- [Download](https://github.com/greenpixels/godot-arch/releases) the latest release for your platform
- Unpack the contents into your project-root (`godot-arch.config.yaml` and `godot-arch`)
- (*optional*) customize the configuration in `godot-arch.config.yaml`
### 3. Usage
- Execute  `godot-arch` either locally in your terminal or preferrably in your CI via e.g. GitHub-Actions

### Requirements

## Rules
These are the default rules that GodotArch checks for. You can of course customize that behaviour via `godot-arch.config.yaml`, but it is recommended to adhere to these defaults. They are a mix of examples in the Godot Documentation and its best-pratices, aswell as inspired by different Godot projects I have seen over times.
### Files
- **MUST** have filename in `snake_case`

### Scenes (.tscn)
- **MUST** be in `(ROOT)/scenes/**` or `(ROOT)/globals/**`
- **MUST** have root node with same name as file in *PascalCase*
- **MUST** be in folder with same name as file

### Scripts (.gd)
- **MUST** be in `(ROOT)/scenes/**`, `(ROOT)/globals/**`, `(ROOT)/resources/**`, or `(ROOT)/test/**`
- **MUST** be in folder with same name as file

### Resources (.tres)
- **MUST** be in `(ROOT)/resources/**`
- **MUST** be in folder with same name as file

### Shaders (.gdshader)
- **MUST** be in `(ROOT)/shaders/**`

### Translations (.translation)
- **MUST** be in `(ROOT)/localization/**`

### Assets
- Images (.png, .jpg, .jpeg, .gif, .webp, .ico) **MUST** be in `(ROOT)/assets/images/**`
- Audio (.mp3, .wave, .ogg, .flac, .aac, .m4a) **MUST** be in `(ROOT)/assets/audio/**`

### Nodes
- Nodes ...
    - **MUST** have their name in *PascalCase*
    - This helps maintain consistency with Godot's built-in node naming conventions

## Configuration

The `godot-arch.config.yaml` file allows you to customize the linter's behavior. Here are the main configuration sections:

### ignorePatterns

Specify file patterns to be ignored by the linter:
>  🚨 **Please note:** All patterns in this configuration need to begin with `./`

```yaml
ignorePatterns:
    overall:  # Ignored by all rules
        - ./godot-arch.exe
        - ./addons/**
        - ...
    # You can also ignore files for specific rules
    "rule-allowed-file-location":
        - ./my_example_file.tscn
    "rule-filename-snake-case":
    "rule-parent-has-same-name":
    "rule-scene-nodes-pascal-case":
```

### allowedFileLocations

Define where specific file types are allowed to be located:
>  🚨 **Please note:** All patterns in this configuration need to begin with `./`

```yaml
allowedFileLocations:
    "./**/*.tscn":  # Scene files
        - ./globals/**
        - ./scenes/**
    "./**/*.gd":    # Script files
        - ./globals/**
        - ./scenes/**
    "./**/*.{png,jpg,jpeg,gif,webp,ico}":  # Image files
        - ./assets/images/**
```

### nodeNamePascalCaseExceptions

Configure exceptions for node naming conventions, especially useful for standard Godot nodes that don't follow PascalCase.

```yaml
nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - VBoxContainer: VerticalBoxContainer
    - HBoxContainer: HorizontalBoxContainer
    # etc...
```

### allowScreamingSnakeCaseInNodeNames

If your project uses translation keys as node names (e.g., childrne of *TabContainer*):

```yaml
# Set to true if you use SCREAMING_SNAKE_CASE for translation keys in node names
allowScreamingSnakeCaseInNodeNames: false
```

Alternatively, you can add specific scenes to the ignore patterns if they contain translation keys.

### shouldPrintSuccess

Controls whether successful rule checks should print output:

```yaml
# Set to true to see output for passing checks
shouldPrintSuccess: true
```

When enabled, this will show confirmations for files that pass the linting rules, not just errors.

## Planned Features

### Maintenance
- [ ] Split code into modules for better maintainability
- [ ] Write unit tests for core functionality

### Additional Tests
- [ ] Scripts and Scenes configured as autoload in project settings are actually inside of autoload and vice versa
- [ ] Root nodes of a scene that contain a script should have that script next to that scene

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.