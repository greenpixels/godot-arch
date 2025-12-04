use crate::{
    rules::rule_filename_snake_case::execute_rule_filename_snake_case,
    tests::mocks::{get_config_mock, get_file_under_check_mock, get_check_results_mock},
};

fn assert_results(file_name: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("resources/my_folder", file_name, "tscn");
    let config = get_config_mock();

    execute_rule_filename_snake_case(&file_under_check, &config, &mut check_results);

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_snake_case() {
    assert_results("file_is_snake_case", 1, 0);
}

#[test]
fn test_rule_should_fail_on_pascal_case() {
    assert_results("FileIsPascalCase", 1, 1);
}
