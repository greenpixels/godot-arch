use crate::{
    models::{
        config::Config, file_under_test::FileUnderTest, test_results::TestResults, warning::Warning,
    },
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
};
use colored::Colorize;
use godot_properties_parser::parsers::parser_scene_file::SceneFile;

pub fn execute_rule_root_node_script_in_same_folder(
    _parsed_scene: &SceneFile,
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(
            config
                .include_patterns
                .root_node_script_in_same_folder
                .to_owned(),
        ),
        Some(
            config
                .ignore_patterns
                .root_node_script_in_same_folder
                .to_owned(),
        ),
        config,
    ) {
        return;
    }

    // No need to test if there arent any nodes to test against, as unlikely as that may be
    if _parsed_scene.nodes.len() <= 0 {
        return;
    }

    let root_node = match _parsed_scene.nodes.first() {
        None => return,
        Some(node) => node,
    };

    let script_resource = match root_node.properties.iter().find(|p| p.key == "script") {
        Some(prop) => &prop.value,
        None => return,
    };
    if !script_resource.starts_with("ExtResource(") {
        test_results.warnings.push(Warning {
            message: format!(
                // TODO https://github.com/greenpixels/godot-arch/issues/5
                "A scene with a non-external script (e.g. \"built-in scripts\") resource can't be parsed, resource is {}",
                script_resource
            ),
            absolute_path: file.absolute_path.to_owned(),
        });
        return;
    }

    let script_resource_id = script_resource
        .trim_start_matches("ExtResource(\"")
        .trim_end_matches("\")");

    let is_valid: bool;
    let optional_external_script_resource = _parsed_scene.ext_resources.iter().find(|&resource| {
        let has_uid = resource.properties.iter().any(|p| p.key == "uid");
        let type_prop = resource.properties.iter().find(|p| p.key == "type");
        let id_prop = resource.properties.iter().find(|p| p.key == "id");

        has_uid
            && type_prop.map(|p| p.value.as_str()) == Some("Script")
            && id_prop.map(|p| p.value.as_str()) == Some(script_resource_id)
    });

    if optional_external_script_resource.is_none() {
        is_valid = false
    } else {
        let script_resource = optional_external_script_resource.unwrap();
        let script_path = match script_resource.properties.iter().find(|p| p.key == "path") {
            None => "".to_owned(),
            Some(prop) => prop.value.trim_start_matches("res://").to_owned(),
        };
        if script_path.is_empty() {
            is_valid = false
        } else {
            is_valid =
                file.relative_path.replace(&file.extension, "gd") == format!("./{}", script_path);
        }
    }

    let validation_output = handle_validation_result(
        is_valid,
        "rule-root-node-script-in-same-folder".to_owned(),
        format!(
            "Script of root node is next to the scene-file for '{}'",
            file.absolute_path.bold(),
        ),
        format!(
            "Expected script of root node to be next to the scene-file for '{}'",
            file.absolute_path.bold()
        ),
        config.should_print_success,
        test_results,
    );
    if validation_output.is_some() {
        println!("{}", validation_output.unwrap())
    }
}
