use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
};
use colored::Colorize;
use glob_match::glob_match;

pub fn execute_rule_parent_has_same_name(
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
    println!("execute_rule_parent_has_same_name");
    // Check if this file should be skipped for parent name validation
    for pattern in config.ignore_patterns.parent_has_same_name.iter() {
        if glob_match(pattern, &file.relative_path) {
            return;
        }
    }

    let parent_option = file.path.parent();
    let mut has_parent_with_same_name = false;
    if parent_option.is_some() {
        let parent = parent_option.unwrap();
        let parent_file_name_option = parent.file_name();
        if parent_file_name_option.is_some() {
            let file_name = parent_file_name_option.unwrap().to_str().unwrap_or("");
            has_parent_with_same_name =
                format!("{}.{}", file_name, file.extension) == file.file_name
        }
    }

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
