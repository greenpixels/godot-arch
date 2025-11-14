use crate::{
    rules::rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

fn assert_results(
    file_name: &str,
    node_name: &str,
    expected_files_tested: i32,
    expected_files_failed: i32,
) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", file_name, "tscn");
    let config = get_config_mock();

    execute_rule_root_node_is_file_name_pascal(
        node_name,
        &file_under_test,
        &config,
        &mut test_results,
    );

    assert_eq!(test_results.files_tested, expected_files_tested);
    assert_eq!(test_results.files_failed, expected_files_failed);
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
