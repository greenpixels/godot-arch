![GodotArch](assets/images/godot-arch-logo.png)

> 🚧👷‍♂️ **Please Note:** GodotArch is still in early development. Please report any bugs and issues you find.

An extremely fast project structure linter four your Godot projects. GodotArch enforces consistent file organization and naming conventions, making them more maintainable and easier to navigate. 

It can check scenes, scripts, nodes, assets, and more. It is also extremely configurable via its `godot-arch.config.yaml`.

## How To Use

In order to use GodotArch you need to have its executable and configuration inside your project root.
### 1. Requirements
- A Godot project developed with `Godot 4.4` or higher 

### 2. Installation & Setup
- [Download](https://github.com/greenpixels/godot-arch/releases) the latest release for your platform
- Unpack the contents into your project-root (`godot-arch.config.yaml` and `godot-arch`)
- (*optional*) customize the configuration in `godot-arch.config.yaml`
### 3. Usage
- Execute  `godot-arch` either locally in your terminal or preferrably in your CI via e.g. "*GitHub-Actions*"

## Available Rules
GodotArch comes with a set of preconfigured rules that are a personal recommendation. You can of course customize that behaviour via the `godot-arch.config.yaml` configuration file.

### Rule: `"Allowed File Location"`
Checks whether matched files are allowed to be in the location they are in. For example, if we want `.tscn` files to only be somewhere inside of a `./scenes` folder and `.png` and `.jpeg` only to be inside of `./images` you can declare the rules as:

```yaml
allowedFileLocations:
    "./**/*.tscn":
        - ./scenes/**
    "./**/*.{png,jpeg}":
        - ./images/**
```

### Rule: `"Filename Snake Case"`
Checks whether the matched files should have their file names be written in snake_case. Examples would be `player_animation_01.png`, `level_01.tscn`, ...

To adhere every file in your project this rule you can set the includePattern for this rule as such:

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**
```

### Rule: `"Parent Has Same Name"`
Checks whether the matches files are inside a folder that has the same name as the file itself. For example, if we always want `.tscn` and `.gd` files to be placed into a folder with the same name you can declare the rule as:

```yaml
includePatterns:
    "rule-filename-snake-case":
        - ./**/*.tscn
        - ./**/*.gd
```

This would result in a file structure that would look like:
- /scenes
  - /player  
    - player&#46;gd
    - player&#46;tscn

### Rule: `"Root Node Is File Name Pascal"`
Checks whether the file-name and the root-node-name as `PascalCase` inside a `.tscn` file match for the matched files. Meaning a scene-file that is named `inventory_grid.tscn` should have a root-node that is named `InventoryGrid.tscn`

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-root-node-is-file-name-pascal":
        - ./**/*.tscn
```

### Rule: `"Scene Nodes Are Pascal Case"`
Checks whether the nodes inside of a scene tree are all written in `PascalCase`

I would recommend turning this rule on for every `.tscn`-file.

```yaml
includePatterns:
    "rule-scene-nodes-pascal-case":
        - ./**/*.tscn
```

## Planned Features

### Maintenance
- [ ] Write unit tests for core functionality

### Additional Tests
- [ ] Scripts and Scenes configured as autoload in project settings are actually inside of autoload and vice versa
- [ ] Root nodes of a scene that contain a script should have that script next to that scene

