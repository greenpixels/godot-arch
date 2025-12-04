use colored::Colorize;
use glob_match::glob_match;

use crate::{
    configuration::config::Config, reporting::check_results::CheckResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_check::FileUnderCheck,
};

pub fn execute_rule_allowed_file_location(
    file: &FileUnderCheck,
    config: &Config,
    check_results: &mut CheckResults,
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

    handle_validation_result(
        is_in_correct_location,
        "rule-allowed-file-location".to_owned(),
        format!("Found {} in correct location", file.file_name.bold()),
        format!(
            "Expected {} to be in {} but found it in {}",
            file.file_name.bold(),
            folders_list.bold(),
            file.relative_path.bold(),
        ),
        check_results,
        file,
    );
}
