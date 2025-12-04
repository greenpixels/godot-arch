use crate::{
    rules::rule_allowed_file_location::execute_rule_allowed_file_location,
    tests::mocks::{get_config_mock, get_file_under_check_mock, get_check_results_mock},
};

fn assert_results(folder: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock(folder, "file_is_snake_case", "tscn");
    let mut config = get_config_mock();
    config
        .allowed_file_locations
        .insert("./**/*.tscn".to_owned(), vec!["./resources/**".to_owned()]);

    execute_rule_allowed_file_location(&file_under_check, &config, &mut check_results);

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_correct_folder() {
    assert_results("resources/my_folder", 1, 0);
}

#[test]
fn test_rule_should_pass_on_incorrect_folder() {
    assert_results("notresources/my_folder", 1, 1);
}
