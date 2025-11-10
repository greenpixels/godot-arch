use crate::{
    rules::rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_rule_should_pass_on_pascal_case_and_same_name() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_root_node_is_file_name_pascal(
        "File",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_on_lower_case_and_same_name() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_root_node_is_file_name_pascal(
        "file",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}

#[test]
fn test_rule_should_fail_on_pascal_case_and_unequal_name() {
    let mut test_results = get_test_results_mock();
    let file_under_test =
        get_file_under_test_mock("resources/my_folder", "super_cool_file", "tscn");
    let config = get_config_mock();
    execute_rule_root_node_is_file_name_pascal(
        "SuperWrongFile",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
