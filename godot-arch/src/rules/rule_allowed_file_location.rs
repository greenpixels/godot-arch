use colored::Colorize;
use glob_match::glob_match;

use crate::{
    config::config::Config, reporting::test_results::TestResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_test::FileUnderTest,
};

pub fn execute_rule_allowed_file_location(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        None,
        Some(config.ignore_patterns.allowed_file_location.to_owned()),
        config,
    ) {
        return;
    }

    let mut is_in_correct_location = false;
    let mut matched_allowed_locations: Vec<String> = vec![];

    // Check if file matches any configured patterns
    for (pattern, locations) in config.allowed_file_locations.iter() {
        if glob_match(pattern, &file.relative_path) {
            for location in locations {
                matched_allowed_locations.push(location.to_owned());
                if glob_match(location, &file.relative_path) {
                    is_in_correct_location = true;
                    break;
                }
            }
        }
    }

    if matched_allowed_locations.is_empty() {
        return;
    }

    let folders_list = matched_allowed_locations.join(" or ");

    let validation_output = handle_validation_result(
        is_in_correct_location,
        "rule-allowed-file-location".to_owned(),
        format!("Found {} in correct location", file.file_name.bold()),
        format!(
            "Expected {} to be in {} but found it in {}",
            file.file_name.bold(),
            folders_list.bold(),
            file.relative_path.bold(),
        ),
        config.should_print_success,
        test_results,
        file,
    );

    if let Some(output) = validation_output {
        println!("{}", output);
    }
}
