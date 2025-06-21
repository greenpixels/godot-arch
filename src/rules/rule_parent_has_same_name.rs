use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
};
use colored::Colorize;

pub fn execute_rule_parent_has_same_name(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(config.include_patterns.parent_has_same_name.to_owned()),
        Some(config.ignore_patterns.parent_has_same_name.to_owned()),
        config,
    ) {
        return;
    }

    let has_parent_with_same_name = file
        .path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|os_str| os_str.to_str())
        .map(|parent_name| {
            let file_stem = file.file_name.split('.').next().unwrap_or("");
            parent_name == file_stem
        })
        .unwrap_or(false);

    handle_validation_result(
        has_parent_with_same_name,
        "rule-parent-has-same-name".to_owned(),
        format!(
            "{} is placed in a folder with the same name",
            file.file_name.bold()
        ),
        format!(
            "Expected {} to be placed in a folder with the same name, but is {}",
            file.file_name.bold(),
            file.relative_path.bold()
        ),
        config,
        test_results,
    );
}
