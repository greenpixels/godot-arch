use crate::{
    rules::rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal,
    tests::mocks::{get_check_results_mock, get_config_mock, get_file_under_check_mock},
};

fn assert_results(
    file_name: &str,
    node_name: &str,
    expected_files_checked: i32,
    expected_checks_failed: i32,
) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("resources/my_folder", file_name, "tscn");
    let config = get_config_mock();

    execute_rule_root_node_is_file_name_pascal(
        node_name,
        &file_under_check,
        &config,
        &mut check_results,
    );

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_pascal_case_and_same_name() {
    assert_results("file", "File", 1, 0);
}

#[test]
fn test_rule_should_fail_on_lower_case_and_same_name() {
    assert_results("file", "file", 1, 1);
}

#[test]
fn test_rule_should_fail_on_pascal_case_and_unequal_name() {
    assert_results("super_cool_file", "SuperWrongFile", 1, 1);
}
