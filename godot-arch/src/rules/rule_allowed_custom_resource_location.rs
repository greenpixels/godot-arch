use colored::Colorize;
use glob_match::glob_match;

use crate::{
    configuration::config::Config, reporting::check_results::CheckResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_check::FileUnderCheck,
};

pub fn execute_rule_allowed_custom_resource_location(
    resource_name: &str,
    file: &FileUnderCheck,
    config: &Config,
    check_results: &mut CheckResults,
) {
    if should_ignore_rule_for_file(
        file,
        None,
        Some(
            config
                .ignore_patterns
                .allowed_custom_resource_location
                .to_owned(),
        ),
        config,
    ) {
        return;
    }

    let mut in_correct_root = false;
    let mut matched_allowed_locations: Vec<String> = vec![];

    // Check if file matches any configured patterns
    for (custom_resource_class_name, locations) in config.allowed_custom_resource_locations.iter() {
        if resource_name.eq(custom_resource_class_name) {
            for location in locations {
                matched_allowed_locations.push(location.to_owned());
                if glob_match(location, &file.relative_path) {
                    in_correct_root = true;
                    break;
                }
            }
        }
    }

    if matched_allowed_locations.is_empty() && !config.should_fail_unmatched_custom_resources {
        return;
    }

    let folders_list = matched_allowed_locations.join(" or ");

    handle_validation_result(
        in_correct_root,
        "rule-allowed-custom-resource-location".to_owned(),
        format!(
            "Found Resource {} in correct location",
            resource_name.bold()
        ),
        if matched_allowed_locations.is_empty() && config.should_fail_unmatched_custom_resources {
            format!(
                "Expected Resource {} to be included in {} configuration",
                resource_name.bold(),
                "allowedCustomResourceLocations".bold()
            )
        } else {
            format!(
                "Expected Resource {} to be in {}, but found it in {}",
                resource_name.bold(),
                folders_list.bold(),
                file.relative_path.bold(),
            )
        },
        check_results,
        file,
    );
}
