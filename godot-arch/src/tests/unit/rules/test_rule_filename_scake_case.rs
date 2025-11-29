use crate::{
    rules::rule_filename_snake_case::execute_rule_filename_snake_case,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

fn assert_results(file_name: &str, expected_files_tested: i32, expected_files_failed: i32) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", file_name, "tscn");
    let config = get_config_mock();

    execute_rule_filename_snake_case(&file_under_test, &config, &mut test_results);

    assert_eq!(test_results.files_tested, expected_files_tested);
    assert_eq!(test_results.files_failed, expected_files_failed);
}

#[test]
fn test_rule_should_pass_on_snake_case() {
    assert_results("file_is_snake_case", 1, 0);
}

#[test]
fn test_rule_should_fail_on_pascal_case() {
    assert_results("FileIsPascalCase", 1, 1);
}
