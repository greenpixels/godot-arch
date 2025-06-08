![GodotArch](assets/images/godot-arch-logo.png)

An executable that lints project structure in your Godot project. GodotArch helps enforce consistent organization and naming conventions across your Godot projects, making them more maintainable and easier to navigate.

## Installation

Currently, you can only build from source (until a proper stable release is published):
```bash
cargo build --release
```
The executable will be available in `target/release/godot-arch`.

## Rules

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
- Images (.png, .jpg, etc.) **MUST** be in `(ROOT)/assets/images/**`
- Audio (.wav, .ogg, etc.) **MUST** be in `(ROOT)/assets/audio/**`

### Nodes
- Nodes ...
    - **MUST** have their name in *PascalCase*
    - This helps maintain consistency with Godot's built-in node naming conventions

## Planned Features

### Maintenance
- [ ] Split code into modules for better maintainability
- [ ] Write unit tests for core functionality

### Additional Tests
- [ ] Scripts and Scenes configured as autoload in project settings are actually inside of autoload and vice versa
- [ ] Root nodes of a scene that contain a script should have that script next to that scene

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.