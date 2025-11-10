use crate::{
    rules::rule_parent_has_same_name::execute_rule_parent_has_same_name,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_rule_should_pass_on_same_name() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("scenes/player", "player", "tscn");
    let config = get_config_mock();
    execute_rule_parent_has_same_name(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_on_unequal_name() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("scenes/something", "player", "tscn");
    let config = get_config_mock();
    execute_rule_parent_has_same_name(&file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
