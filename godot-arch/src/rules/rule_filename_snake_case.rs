use colored::Colorize;
use convert_case::{Case, Casing};

use crate::{
    configuration::config::Config, reporting::test_results::TestResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_test::FileUnderTest,
};

pub fn execute_rule_filename_snake_case(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(config.include_patterns.filename_snake_case.to_owned()),
        Some(config.ignore_patterns.filename_snake_case.to_owned()),
        config,
    ) {
        return;
    }

    let is_valid = file.file_name.is_case(Case::Snake);

    let validation_output = handle_validation_result(
        is_valid,
        "rule-filename-snake-case".to_owned(),
        format!(
            "{} uses correct lowercase snake_case naming convention",
            file.file_name.bold()
        ),
        format!(
            "Expected lowercase snake_case for {} - should be {}, but got {}",
            file.file_name.bold(),
            file.file_name.to_case(Case::Snake).bold(),
            file.file_name.bold()
        ),
        config.should_print_success,
        test_results,
        file,
    );
    if validation_output.is_some() {
        println!("{}", validation_output.unwrap())
    }
}
