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

## Linting Rules

GodotArch comes with a set of preconfigured rules and defaults that are a personal preference. You can of course completely customize that behavior via the `godot-arch.config.yaml` configuration file by adding or removing rules from `includePatterns` and `ignorePatterns`.

### Rule: `"Allowed File Location"`

Checks whether matched files are allowed to be in the location they are in. For example, if we want `.tscn` files to only be somewhere inside of a `./scenes` folder and we also want `.png` and `.jpeg` only to be inside of `./images`, then we can configure it as:

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.{png,jpeg}":
        - ./images/**
```

> This rule does not have an `includePattern` to configure, only an `ignorePattern:`. That is because the configuration itself already declares an inclusion implicitly.

#### How to mitigate on failure

- Move the affected file into a folder matching the configured pattern
- Reconfigure `allowedFileLocations:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-allowed-file-location:`

### Rule: `"Filename Snake Case"`

Checks whether the matched files should have their file names be written in snake_case. Examples would be `player_animation_01.png`, `level_01.tscn`, ...

To have every file in your project adhere to this rule you can set the `includePattern` for this rule as such:

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**
```

#### How to mitigate on failure

- Rename the affected file to *snake_case*
- Reconfigure `includePatterns:` -> `rule-filename-snake-case:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-filename-snake-case:`

### Rule: `"Parent Has Same Name"`

Checks whether the matched files are inside a folder that has the same name as the file itself. For example, if we always want `.tscn` and `.gd` files to be placed into a folder with the same name you can configure it as:

```yaml
includePatterns:
    "rule-parent-has-same-name":
        - ./**/*.tscn
        - ./**/*.gd
```

This would result in a file structure that would look like:

- /scenes
  - /player  
    - player&#46;gd
    - player&#46;tscn

#### How to mitigate on failure

- Rename the affected file to fit the folder above
- Rename the folder to fit the affected file
- Reconfigure `includePatterns:` -> `rule-parent-has-same-name:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-parent-has-same-name:`

### Rule: `"Root Node Is File Name Pascal"`

Checks whether the file name and root node name (both as `PascalCase`) match inside a `.tscn` file. Meaning a scene file that is named `inventory_grid.tscn` should have a root node that is named `InventoryGrid`

> All non-scene files are automatically ignored as they don't have any nodes.

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./**/*.tscn
```

#### How to mitigate on failure

- Rename the affected node inside the tree to be the PascalCase version of its scene file name
- Rename the scene file to be the snake_case version of the affected node
- Reconfigure `includePatterns:` -> `rule-root-node-is-file-name-pascal:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-root-node-is-file-name-pascal:`

### Rule: `"Scene Nodes Are Pascal Case"`

Checks whether the nodes inside a scene tree are all written in `PascalCase`

> All non-scene files are automatically ignored as they don't have any nodes.

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-scene-nodes-pascal-case":
        - ./**/*.tscn
```

There are cases in which using *PascalCase* feels incorrect, especially when using abbreviations. In fact, some native GDScript-Nodes do not actually conform to PascalCase (e.g. GPUParticle2D). For those cases you can add an exception via `nodeNamePascalCaseExceptions:`, which remaps the non-conformal NodeName to a conformal NodeName. This can be either a part of a name or the full name of node. This does not actually rename your node, but lets the test pass without issues. For example:

```yaml
nodeNamePascalCaseExceptions:
    - GPU: Gpu
    - CPUParticle2D: CpuParticle2D
```

#### How to mitigate on failure

- Rename the affected nodes to *PascalCase*
- Reconfigure or add `nodeNamePascalCaseExceptions:` and add a remap to the affected node name
- Reconfigure `includePatterns:` -> `rule-scene-nodes-pascal-case:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-scene-nodes-pascal-case:`

### Rule: `"Node Depth Fits Max Depth"`

Checks how deeply a node is nested in a scene tree. Should not exceed `maxNodeDepth`, which you can optionally set in your configuration. (default is `4`).

> All non-scene files are automatically ignored as they don't have any nodes.

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-node-depth-fits-max-depth":
        - ./**/*.tscn

maxNodeDepth: 3
```

#### How to mitigate on failure

- Refactor the affected node branch into a new scene
- Reconfigure `maxNodeDepth:` to reflect your preferred changes
- Reconfigure `includePatterns:` -> `rule-node-depth-fits-max-depth:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-node-depth-fits-max-depth:`

### Rule: `"Root Node Script In Same Folder"`

Checks whether the script of the root node of a scene is placed next to that scene. Let's assume you have a `Player.tscn`. If the root node of that scene has a script, then that script should be `Player.gd`. And that script should be placed next to `Player.tscn`, resulting in:

- player/
  - player.gd
  - player.tscn

> All non-scene files are automatically ignored as they don't have any nodes.

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-root-node-script-in-same-folder":
        - ./**/*.tscn
```

#### How to mitigate on failure

- Move the script of the affected root node next to the root node
- Reconfigure `includePatterns:` -> `rule-root-node-script-in-same-folder:` to reflect your preferred changes
- Add this file or all files to `ignorePatterns:` -> `rule-root-node-script-in-same-folder:`

## Planned Features

- Scripts and Scenes configured as autoload in project settings are actually inside of autoload and vice versa
