use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
    util::{
        parse_scene_file::ParsedSceneEntry,
        should_ignore_rule_for_file::should_ignore_rule_for_file,
    },
};
use colored::Colorize;

pub fn execute_rule_node_depth_fits_max_depth(
    node: &ParsedSceneEntry,
    node_name: &str,
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(config.include_patterns.node_depth_fits_max_depth.to_owned()),
        Some(config.ignore_patterns.node_depth_fits_max_depth.to_owned()),
        config,
    ) {
        return;
    }

    if !node.header_properties.contains_key("parent") {
        return;
    }

    let parent = node.header_properties.get("parent").unwrap();
    let actual_depth = parent.split("/").collect::<Vec<&str>>().len();

    let is_valid = actual_depth <= config.max_node_depth;

    let validation_output = handle_validation_result(
        is_valid,
        "rule-node-depth-fits-max-depth".to_owned(),
        format!(
            "{} adheres with a depth of {:0} to the maximum node depth of {:0}",
            node_name.bold(),
            actual_depth,
            config.max_node_depth
        ),
        format!(
            "{} depth of {:0} in {} exceeds the maximum node depth of {:0}",
            node_name.bold(),
            actual_depth.to_string().red(),
            file.relative_path.bold(),
            config.max_node_depth.to_string().green(),
        ),
        config.should_print_success,
        test_results,
    );
    if validation_output.is_some() {
        println!("{}", validation_output.unwrap())
    }
}
