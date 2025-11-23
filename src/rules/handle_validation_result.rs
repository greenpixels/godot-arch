use colored::Colorize;

pub fn handle_validation_result(
    is_success: bool,
    rule_name: String,
    success_message: String,
    error_message: String,
    should_print_success: bool,
    test_results: &mut crate::models::test_results::TestResults,
) -> Option<String> {
    test_results.files_tested += 1;
    if is_success {
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
    test_results.files_failed += 1;
    Some(format!(
        "{} ({}): {}",
        "Test Failed".red(),
        rule_name.bright_black(),
        error_message
    ))
}
