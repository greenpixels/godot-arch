use crate::{
    rules::rule_filename_snake_case::execute_rule_filename_snake_case,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_rule_should_pass_on_snake_case() {
    let mut test_results = get_test_results_mock();
    let file_under_test =
        get_file_under_test_mock("resources/my_folder", "file_is_snake_case", "tscn");
    let config = get_config_mock();
    execute_rule_filename_snake_case(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_on_pascal_case() {
    let mut test_results = get_test_results_mock();
    let file_under_test =
        get_file_under_test_mock("resources/my_folder", "FileIsPascalCase", "tscn");
    let config = get_config_mock();
    execute_rule_filename_snake_case(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
