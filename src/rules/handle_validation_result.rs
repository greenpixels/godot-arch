use colored::Colorize;

use crate::models::{file_under_test::FileUnderTest, report_entry::ReportEntry};

fn strip_ansi_codes(text: &str) -> String {
    let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    return ansi_regex.replace_all(text, "").to_string();
}

pub fn handle_validation_result(
    is_success: bool,
    rule_name: String,
    success_message: String,
    error_message: String,
    should_print_success: bool,
    test_results: &mut crate::models::test_results::TestResults,
    file_under_test: &FileUnderTest,
) -> Option<String> {
    test_results.files_tested += 1;
    if is_success {
        test_results.successful_reports.push(ReportEntry {
            absolute_file_path: file_under_test.absolute_path.clone(),
            message: strip_ansi_codes(&success_message.clone()),
            rule_name: rule_name.clone(),
        });

        if !should_print_success {
            return None;
        }
        return Some(format!(
            "{} ({}): {}",
            "Test Succesful".green(),
            rule_name.bright_black(),
            success_message,
        ));
    }
    test_results.failed_reports.push(ReportEntry {
        absolute_file_path: file_under_test.absolute_path.clone(),
        message: strip_ansi_codes(&error_message.clone()),
        rule_name: rule_name.clone(),
    });
    test_results.files_failed += 1;
    Some(format!(
        "{} ({}): {}",
        "Test Failed".red(),
        rule_name.bright_black(),
        error_message
    ))
}
