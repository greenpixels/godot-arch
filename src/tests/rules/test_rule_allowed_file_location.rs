use crate::{
    rules::rule_allowed_file_location::execute_rule_allowed_file_location,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_rule_should_pass_on_correct_folder() {
    let mut test_results = get_test_results_mock();
    let file_under_test =
        get_file_under_test_mock("resources/my_folder", "file_is_snake_case", "tscn");
    let mut config = get_config_mock();
    config
        .allowed_file_locations
        .insert("./**/*.tscn".to_owned(), vec!["./resources/**".to_owned()]);
    execute_rule_allowed_file_location(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_pass_on_incorrect_folder() {
    let mut test_results = get_test_results_mock();
    let file_under_test =
        get_file_under_test_mock("notresources/my_folder", "file_is_snake_case", "tscn");
    let mut config = get_config_mock();
    config
        .allowed_file_locations
        .insert("./**/*.tscn".to_owned(), vec!["./resources/**".to_owned()]);
    execute_rule_allowed_file_location(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
