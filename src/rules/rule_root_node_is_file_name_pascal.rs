use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
};
use colored::Colorize;
use convert_case::{Case, Casing};

pub fn execute_rule_root_node_is_file_name_pascal(
    node_name: &str,
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(
            config
                .include_patterns
                .root_node_is_file_name_pascal
                .to_owned(),
        ),
        Some(
            config
                .ignore_patterns
                .root_node_is_file_name_pascal
                .to_owned(),
        ),
        config,
    ) {
        return;
    }
    // TODO: Currently we aren't using node_name_pascal_case_exceptions here
    let file_name_as_pascal_case = file
        .file_name
        .replace(&format!(".{}", file.extension).to_string(), "")
        .to_case(Case::Pascal);
    let root_node_has_same_name = file_name_as_pascal_case == node_name;
    let validation_output = handle_validation_result(
        root_node_has_same_name,
        "rule-root-node-is-file-name-pascal".to_owned(),
        format!(
            "Root node of {} is {}",
            file.file_name.bold(),
            file_name_as_pascal_case.bold()
        ),
        format!(
            "Expected root node of {} to be {}, but was {}",
            file.file_name.bold(),
            file_name_as_pascal_case.bold(),
            node_name.bold()
        ),
        config.should_print_success,
        test_results,
    );
    if validation_output.is_some() {
        println!("{}", validation_output.unwrap())
    }
}
