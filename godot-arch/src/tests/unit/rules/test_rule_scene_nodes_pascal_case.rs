use std::collections::HashMap;

use crate::{
    configuration::config::Config,
    rules::rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case,
    tests::mocks::{get_check_results_mock, get_config_mock, get_file_under_check_mock},
};

fn assert_results(
    node_name: &str,
    config_modifier: impl FnOnce(&mut Config),
    expected_files_checked: i32,
    expected_checks_failed: i32,
) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config_modifier(&mut config);

    execute_rule_scene_needs_pascal_case(node_name, &file_under_check, &config, &mut check_results);

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_on_pascal_case() {
    assert_results("PascalCase", |_| {}, 1, 0);
}

#[test]
fn test_rule_should_fail_on_screaming_snake_case() {
    assert_results("SCREAMING_SNAKE_CASE", |_| {}, 1, 1);
}

#[test]
fn test_rule_should_pass_on_screaming_snake_case_when_screaming_snake_case_is_allowed() {
    assert_results(
        "SCREAMING_SNAKE_CASE",
        |config| config.allow_screaming_snake_case_in_node_names = true,
        1,
        0,
    );
}

#[test]
fn test_rule_should_pass_with_exceptions() {
    assert_results(
        "GPUNode",
        |config| {
            let mut exceptions: HashMap<String, String> = HashMap::new();
            exceptions.insert("GPU".to_owned(), "Gpu".to_owned());
            config.node_name_pascal_case_exceptions.push(exceptions);
        },
        1,
        0,
    );
}

#[test]
fn test_rule_should_fail_without_exceptions() {
    assert_results("GPUNode", |_| {}, 1, 1);
}

#[test]
fn test_rule_should_fail_on_snake_case() {
    assert_results("snake_case", |_| {}, 1, 1);
}

#[test]
fn test_rule_should_be_ignored() {
    assert_results(
        "snake_case",
        |config| {
            config
                .ignore_patterns
                .scene_nodes_pascal_case
                .insert(0, "./**".to_owned());
        },
        0,
        0,
    );
}
