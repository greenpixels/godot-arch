use colored::Colorize;
use convert_case::{Case, Casing};
use glob_match::glob_match;

use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
};

pub fn execute_rule_filename_snake_case(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    // Check if this file should be skipped for filename validation
    for pattern in config.ignore_patterns.filename_snake_case.iter() {
        if glob_match(pattern, &file.relative_path) {
            return;
        }
    }

    let is_valid = file.file_name.is_case(Case::Snake);

    handle_validation_result(
        is_valid,
        "rule-filename-snake-case".to_owned(),
        format!(
            "{} uses correct lowercase snake_case naming convention",
            file.file_name.bold()
        ),
        format!(
            "Expected lowercase snake_case for {}, but got {}",
            file.file_name.bold(),
            file.file_name.bold()
        ),
        config,
        test_results,
    );
}
