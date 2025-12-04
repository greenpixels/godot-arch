use crate::{
    rules::rule_allowed_custom_resource_location::execute_rule_allowed_custom_resource_location,
    tests::mocks::{get_check_results_mock, get_config_mock, get_file_under_check_mock},
};

fn assert_results(folder: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock(folder, "file", "tres");
    let mut config = get_config_mock();
    config
        .allowed_custom_resource_locations
        .insert("File".to_owned(), vec!["./files/**".to_owned()]);
    config.should_fail_unmatched_custom_resources = true;
    execute_rule_allowed_custom_resource_location(
        "File",
        &file_under_check,
        &config,
        &mut check_results,
    );

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_correct_folder() {
    assert_results("files/all", 1, 0);
}

#[test]
fn test_rule_should_pass_on_incorrect_folder() {
    assert_results("notfiles/all", 1, 1);
}

#[test]
fn test_rule_should_fail_on_unmatched_custom_resource() {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("files/all", "file", "tres");
    let mut config = get_config_mock();
    config.should_fail_unmatched_custom_resources = true;
    execute_rule_allowed_custom_resource_location(
        "UnmatchedFile",
        &file_under_check,
        &config,
        &mut check_results,
    );

    assert_eq!(check_results.files_checked, 1);
    assert_eq!(check_results.files_failed, 1);
}
