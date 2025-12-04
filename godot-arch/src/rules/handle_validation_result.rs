use crate::reporting::report_entry::ReportEntry;
use crate::validation::file_under_check::FileUnderCheck;

fn strip_ansi_codes(text: &str) -> String {
    let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_regex.replace_all(text, "").to_string()
}

pub fn handle_validation_result(
    is_success: bool,
    rule_name: String,
    success_message: String,
    error_message: String,
    check_results: &mut crate::reporting::check_results::CheckResults,
    file_under_check: &FileUnderCheck,
) {
    check_results.files_checked += 1;
    if is_success {
        check_results.successful_reports.push(ReportEntry {
            absolute_file_path: file_under_check.absolute_path.clone(),
            message: strip_ansi_codes(&success_message.clone()),
            rule_name: rule_name.clone(),
        });
    } else {
        check_results.failed_reports.push(ReportEntry {
            absolute_file_path: file_under_check.absolute_path.clone(),
            message: strip_ansi_codes(&error_message.clone()),
            rule_name: rule_name.clone(),
        });
        check_results.files_failed += 1;
    }
}
