![GodotArch](assets/images/godot-arch-logo.png)

> 🚧👷‍♂️ **Please Note:** GodotArch is still in early development. Please report any bugs and issues you find.

An extremely fast and configurable project structure linter for your Godot projects. GodotArch enforces consistent file organization and naming conventions, making them more maintainable and easier to navigate.

It can check scenes, scripts, nodes, assets, and more. It is also extremely configurable via its `godot-arch.config.yaml`.

## How To Use

In order to use GodotArch you need to have its `godot-arch` executable and a `godot-arch.config.yaml` configuration file at your project root.

### 1. Requirements

- A Godot project developed with `Godot 4.4` or higher
  - (Earlier versions may work, but are untested.)

### 2. Installation & Setup

- [Download](https://github.com/greenpixels/godot-arch/releases) the latest release for your platform
- Unpack the contents into your project-root (`godot-arch.config.yaml` and `godot-arch`)
- (*optional*) customize the configuration in `godot-arch.config.yaml`

### 3. Usage

- Run `./godot-arch` (or `./godot-arch.exe` for Windows) either locally in your terminal or in your CI via e.g. "*GitHub-Actions*"

## Documentation

- **[Configuration Reference](./docs/configuration.md)** - Complete guide to all configuration options
- **[Glob Patterns Guide](./docs/glob-patterns.md)** - Learn how to write glob patterns for file matching

## Linting Rules

GodotArch comes with with a default configuration file. All rules can be configured via `includePatterns` and `ignorePatterns` in your `godot-arch.config.yaml`. It's recommended to start with the default configuration and adjust it based on your project's specific needs.

### Available Rules

- **[Allowed File Location](./docs/rules/rule-allowed-file-location.md)** - Restrict files to specific directories
- **[Filename Snake Case](./docs/rules/rule-filename-snake-case.md)** - Enforce snake_case file names
- **[Parent Has Same Name](./docs/rules/rule-parent-has-same-name.md)** - Files must be in a folder with matching name
- **[Scene Nodes Pascal Case](./docs/rules/rule-scene-nodes-pascal-case.md)** - Enforce PascalCase node names
- **[Root Node Is File Name Pascal](./docs/rules/rule-root-node-is-file-name-pascal.md)** - Scene root node matches file name
- **[Root Node Script In Same Folder](./docs/rules/rule-root-node-script-in-same-folder.md)** - Root node scripts next to scenes
- **[Node Depth Fits Max Depth](./docs/rules/rule-node-depth-fits-max-depth.md)** - Limit node nesting depth
