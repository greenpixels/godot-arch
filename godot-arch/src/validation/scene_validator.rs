use godot_properties_parser::parsers::parser_scene_file::SceneFile;

use crate::config::config::Config;
use crate::reporting::test_results::TestResults;
use crate::rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth;
use crate::rules::rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal;
use crate::rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder;
use crate::rules::rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case;
use crate::validation::file_under_test::FileUnderTest;

pub fn validate_scene_file(
    parsed_scene_file: SceneFile,
    file: &FileUnderTest,
    test_results: &mut TestResults,
    config: &Config,
) {
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
