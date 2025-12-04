use colored::Colorize;
use convert_case::{Case, Casing};

use crate::{
    configuration::config::Config, reporting::check_results::CheckResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_check::FileUnderCheck,
};

pub fn execute_rule_scene_needs_pascal_case(
    node_name: &str,
    file: &FileUnderCheck,
    config: &Config,
    check_results: &mut CheckResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(config.include_patterns.scene_nodes_pascal_case.to_owned()),
        Some(config.ignore_patterns.scene_nodes_pascal_case.to_owned()),
        config,
    ) {
        return;
    }
    let mut node_name_to_test = node_name.to_owned();
    for entry in config.node_name_pascal_case_exceptions.iter() {
        if let Some((uppercase, pascal_case)) = entry.iter().next() {
            node_name_to_test = node_name_to_test.replace(uppercase, pascal_case);
        }
    }

    let mut is_valid = node_name_to_test.is_case(Case::Pascal);
    if config.allow_screaming_snake_case_in_node_names && !is_valid {
        is_valid = node_name_to_test.is_case(Case::UpperSnake);
    }

    handle_validation_result(
        is_valid,
        "rule-scene-nodes-pascal-case".to_owned(),
        format!(
            "Used correct naming-convention for node {} in scene '{}'",
            node_name.bold(),
            file.file_name.bold()
        ),
        format!(
            "Expected PascalCase{} naming-convention for node in '{}', but was {}",
            if config.allow_screaming_snake_case_in_node_names {
                " or SCREAMING_SNAKE_CASE"
            } else {
                ""
            },
            file.file_name.bold(),
            node_name.bold(),
        ),
        check_results,
        file,
    );
}
