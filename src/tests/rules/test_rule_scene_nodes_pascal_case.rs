use std::collections::HashMap;

use crate::{
    rules::rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case,
    tests::mocks::{get_config_mock, get_file_under_test_mock, get_test_results_mock},
};

#[test]
fn test_rule_should_pass_on_pascal_case() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_scene_needs_pascal_case(
        "PascalCase",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_on_screaming_snake_case() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_scene_needs_pascal_case(
        "SCREAMING_SNAKE_CASE",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}

#[test]
fn test_rule_should_pass_on_screaming_snake_case_when_screaming_snake_case_is_allowed() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config.allow_screaming_snake_case_in_node_names = true;
    execute_rule_scene_needs_pascal_case(
        "SCREAMING_SNAKE_CASE",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_pass_with_exceptions() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    let mut exceptions: HashMap<String, String> = HashMap::new();
    exceptions.insert("GPU".to_owned(), "Gpu".to_owned());
    config.node_name_pascal_case_exceptions.push(exceptions);
    execute_rule_scene_needs_pascal_case("GPUNode", &file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_without_exceptions() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_scene_needs_pascal_case("GPUNode", &file_under_test, &config, &mut test_results);
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}

#[test]
fn test_rule_should_fail_on_snake_case() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    execute_rule_scene_needs_pascal_case(
        "snake_case",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}

#[test]
fn test_rule_should_be_ignored() {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config
        .ignore_patterns
        .scene_nodes_pascal_case
        .insert(0, "./**".to_owned());
    execute_rule_scene_needs_pascal_case(
        "snake_case",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 0);
    assert_eq!(test_results.files_failed, 0);
}
