use godot_properties_parser::parsers::parser_scene_file::SceneFile;

use crate::{
    configuration::config::Config,
    reporting::check_results::CheckResults,
    rules::{
        rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth,
        rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal,
        rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder,
        rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case,
    },
    validation::file_under_check::FileUnderCheck,
};

pub fn validate_scene_file(
    parsed_scene_file: SceneFile,
    file: &FileUnderCheck,
    check_results: &mut CheckResults,
    config: &Config,
) {
    let mut is_root_node = true;
    execute_rule_root_node_script_in_same_folder(&parsed_scene_file, file, config, check_results);

    for node in &parsed_scene_file.nodes {
        let node_name = match node.properties.iter().find(|p| p.key == "name") {
            None => return,
            Some(prop) => &prop.value,
        };

        if is_root_node {
            execute_rule_root_node_is_file_name_pascal(
                node_name.as_str(),
                file,
                config,
                check_results,
            );
            is_root_node = false;
        }
        execute_rule_scene_needs_pascal_case(node_name.as_str(), file, config, check_results);
        execute_rule_node_depth_fits_max_depth(node, node_name, file, config, check_results);
    }
}
