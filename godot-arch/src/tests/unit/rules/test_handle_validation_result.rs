use crate::{
    rules::handle_validation_result,
    tests::mocks::{get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_should_correctly_handle_failed_validation() {
    let mut test_results = get_test_results_mock();
    let rule_name = "test-rule".to_string();
    let error_message = "This is an error message".to_string();

    handle_validation_result::handle_validation_result(
        false,
        rule_name.clone(),
        String::new(),
        error_message.clone(),
        &mut test_results,
        &get_file_under_test_mock("folder", "file_is_snake_case", "tscn"),
    );

    assert_eq!(test_results.files_failed, 1);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.failed_reports.len(), 1);
    assert_eq!(test_results.successful_reports.len(), 0);
    assert_eq!(test_results.failed_reports[0].rule_name, rule_name);
    assert_eq!(test_results.failed_reports[0].message, error_message);
}

#[test]
fn test_should_correctly_handle_successful_validation() {
    let mut test_results = get_test_results_mock();
    let rule_name = "test-rule".to_string();
    let success_message = "This is a success message".to_string();

    handle_validation_result::handle_validation_result(
        true,
        rule_name.clone(),
        success_message.clone(),
        String::new(),
        &mut test_results,
        &get_file_under_test_mock("folder", "file_is_snake_case", "tscn"),
    );

    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
    assert_eq!(test_results.failed_reports.len(), 0);
    assert_eq!(test_results.successful_reports.len(), 1);
    assert_eq!(test_results.successful_reports[0].rule_name, rule_name);
    assert_eq!(test_results.successful_reports[0].message, success_message);
}
