![GodotArch](assets/images/godot-arch-logo.png)

An executable that lints project structure in your Godot project.

## Project Structure Requirements

- Everything ...
    - **MUST** have its filename in `snake_case`
- Scenes ...
    - **MUST** be in `(ROOT)/scenes/**` or `(ROOT)/autoload/**`
    - **MUST** have a root node with the same name as the file, but in *PascalCase*
    - **MUST** be in a folder that has the same name as the file

- Scripts ...
    - **MUST** be in `(ROOT)/scenes/**` or `(ROOT)/autoload/**`
    - **MUST** be in a folder that has the same name as the file
- Nodes ...
  - **MUST** have their name as *PascalCase*
- Resources ...
    - **MUST** be in `(ROOT)/resources/**`
    - **MUST** be in a folder that has the same name as the file

## Roadmap

### Maintenance
- [ ] Split code into modules
- [ ] Write unit tests 

### Usability
- [ ] Make it possible to add in a configuration that changes behaviour
    - [ ] Make it possible to add glob patterns for ignoring files
    - [ ] Add exceptions for NodeName-Linting (e.g *URL*, *HBoxContainer*, ...)
    - [ ] Add an option to allow SCREAMING_SNAKE_CASE in NodeNames for translation-keys

### Tests to Implement
- [ ] Test whether scenes inside `/autoload` are actually inside the autoload configuration
- [ ] Test the top-level node in a scene has a script and if so, whether that script is right next to the scene file
- [ ] Test whether all folders are snake_case