use crate::{
    rules::rule_parent_has_same_name::execute_rule_parent_has_same_name,
    tests::mocks::{get_config_mock, get_file_under_check_mock, get_check_results_mock},
};

fn assert_results(folder: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock(folder, "player", "tscn");
    let config = get_config_mock();

    execute_rule_parent_has_same_name(&file_under_check, &config, &mut check_results);

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_same_name() {
    assert_results("scenes/player", 1, 0);
}

#[test]
fn test_rule_should_fail_on_unequal_name() {
    assert_results("scenes/something", 1, 1);
}
