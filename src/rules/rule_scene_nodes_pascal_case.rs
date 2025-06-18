use colored::Colorize;
use convert_case::{Case, Casing};

use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
};

pub fn execute_rule_scene_needs_pascal_case(
    node_name: &str,
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    let node_name_to_test = node_name.to_owned();
    let mut is_valid = node_name_to_test.is_case(Case::Pascal);
    if config.allow_screaming_snake_case_in_node_names && !is_valid {
        is_valid =
            node_name_to_test.is_case(Case::Upper) && node_name_to_test.is_case(Case::UpperSnake);
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
            "Expected PascalCase{} naming-convention, but was {} in filename for '{}'",
            if config.allow_screaming_snake_case_in_node_names {
                " or SCREAMING_SNAKE_CASE"
            } else {
                ""
            },
            node_name.bold(),
            file.file_name.bold()
        ),
        config,
        test_results,
    );
}
