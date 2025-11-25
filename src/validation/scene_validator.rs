use godot_properties_parser::parse_scene_file;

use crate::interface::config::Config;
use crate::reporting::test_results::TestResults;
use crate::reporting::warning::Warning;
use crate::rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth;
use crate::rules::rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal;
use crate::rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder;
use crate::rules::rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case;
use crate::validation::file_under_test::FileUnderTest;

pub fn validate_scene_nodes(file: &FileUnderTest, test_results: &mut TestResults, config: &Config) {
    let file_content = match std::fs::read_to_string(&file.absolute_path) {
        Ok(content) => content,
        Err(_) => {
            test_results.warnings.push(Warning {
                absolute_path: file.absolute_path.clone(),
                message: String::from("Unable to read scene file"),
            });
            return;
        }
    };

    let parsed_scene_file = match parse_scene_file(&file_content) {
        Err(_warning) => {
            test_results.warnings.push(Warning {
                absolute_path: file.absolute_path.clone(),
                message: String::from("Unable to parse scene file"),
            });
            return;
        }
        Ok((_, scene_file)) => scene_file,
    };

    let mut is_root_node = true;
    execute_rule_root_node_script_in_same_folder(&parsed_scene_file, file, config, test_results);

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
                test_results,
            );
            is_root_node = false;
        }
        execute_rule_scene_needs_pascal_case(node_name.as_str(), file, config, test_results);
        execute_rule_node_depth_fits_max_depth(node, node_name, file, config, test_results);
    }
}
