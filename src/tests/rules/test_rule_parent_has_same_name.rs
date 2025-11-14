use crate::{
    rules::rule_parent_has_same_name::execute_rule_parent_has_same_name,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

fn assert_results(folder: &str, expected_files_tested: i32, expected_files_failed: i32) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock(folder, "player", "tscn");
    let config = get_config_mock();

    execute_rule_parent_has_same_name(&file_under_test, &config, &mut test_results);

    assert_eq!(test_results.files_tested, expected_files_tested);
    assert_eq!(test_results.files_failed, expected_files_failed);
}

#[test]
fn test_rule_should_pass_on_same_name() {
    assert_results("scenes/player", 1, 0);
}

#[test]
fn test_rule_should_fail_on_unequal_name() {
    assert_results("scenes/something", 1, 1);
}
