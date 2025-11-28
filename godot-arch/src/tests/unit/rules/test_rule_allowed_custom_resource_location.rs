use crate::{
    rules::rule_allowed_custom_resource_location::execute_rule_allowed_custom_resource_location,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

fn assert_results(folder: &str, expected_files_tested: i32, expected_files_failed: i32) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock(folder, "file", "tres");
    let mut config = get_config_mock();
    config
        .allowed_custom_resource_locations
        .insert("File".to_owned(), vec!["./files/**".to_owned()]);

    execute_rule_allowed_custom_resource_location(
        "File",
        &file_under_test,
        &config,
        &mut test_results,
    );

    assert_eq!(test_results.files_tested, expected_files_tested);
    assert_eq!(test_results.files_failed, expected_files_failed);
}

#[test]
fn test_rule_should_pass_on_correct_folder() {
    assert_results("files/all", 1, 0);
}

#[test]
fn test_rule_should_pass_on_incorrect_folder() {
    assert_results("notfiles/all", 1, 1);
}
