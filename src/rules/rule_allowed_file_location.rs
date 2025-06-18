use colored::Colorize;
use glob_match::glob_match;

use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
};

pub fn execute_rule_allowed_file_location(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(config.ignore_patterns.allowed_file_location.to_owned()),
        None,
    ) {
        return;
    }

    let mut in_correct_root = false;
    let mut can_skip = true;
    let mut matched_locations: Vec<String> = vec![];

    for (pattern, locations) in config.allowed_file_locations.iter() {
        if glob_match(pattern, &file.relative_path) {
            can_skip = false;
            for location in locations {
                matched_locations.push(location.to_owned());
                if glob_match(location, &file.relative_path) {
                    in_correct_root = true;
                }
            }
        }
    }
    if can_skip {
        return;
    }
    let folders_list = matched_locations.join(" or ");

    handle_validation_result(
        in_correct_root,
        "rule-allowed-file-location".to_owned(),
        format!("Found {} in correct location", file.file_name.bold()),
        format!(
            "Expected {} to be in {} but found it in {}",
            file.file_name.bold(),
            folders_list.bold(),
            file.relative_path.bold(),
        ),
        config,
        test_results,
    );
}
