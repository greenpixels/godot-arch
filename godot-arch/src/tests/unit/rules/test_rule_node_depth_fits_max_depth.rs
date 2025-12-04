use crate::{
    rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth,
    tests::mocks::{
        get_check_results_mock, get_config_mock, get_file_under_check_mock,
        get_scene_node_mock_with_external_script,
    },
};
use godot_properties_parser::parsers::parser_property::UntypedProperty;

fn assert_results(parent_path: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config.max_node_depth = 3;

    let mut parsed_node = get_scene_node_mock_with_external_script("File", "myverycooltestid");
    parsed_node.properties.push(UntypedProperty {
        key: String::from("parent"),
        value: String::from(parent_path),
    });

    execute_rule_node_depth_fits_max_depth(
        &parsed_node,
        "NodeName",
        &file_under_check,
        &config,
        &mut check_results,
    );

    assert_eq!(
        check_results.files_checked, expected_files_checked,
        "files_checked mismatch"
    );
    assert_eq!(
        check_results.files_failed, expected_checks_failed,
        "checks_failed mismatch"
    );
}

#[test]
fn test_rule_should_pass_with_node_having_no_parent() {
    assert_results(".", 1, 0);
}

#[test]
fn test_rule_should_pass_with_node_depth_less_than_max() {
    assert_results("HBoxContainer/Control", 1, 0);
}

#[test]
fn test_rule_should_fail_with_node_depth_more_than_max() {
    assert_results("HBoxContainer/Control/Node2D/CPUParticle2D/Control", 1, 1);
}
