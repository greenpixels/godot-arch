use crate::{
    rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth,
    tests::mocks::{
        get_config_mock, get_file_under_test_mock, get_scene_node_mock_with_external_script,
        get_test_results_mock,
    },
};

const TEST_ID: &str = "myverycooltestid";
const NODE_NAME: &str = "NodeName";
const TEST_FOLDER: &str = "resources/my_folder";
const TEST_FILE: &str = "file";
const MAX_DEPTH: usize = 3;

fn assert_results(parent_path: &str, expected_files_tested: i32, expected_files_failed: i32) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock(TEST_FOLDER, TEST_FILE, "tscn");
    let mut config = get_config_mock();
    config.max_node_depth = MAX_DEPTH;

    let mut parsed_node = get_scene_node_mock_with_external_script("File", TEST_ID);
    parsed_node
        .header_properties
        .insert(String::from("parent"), String::from(parent_path));

    execute_rule_node_depth_fits_max_depth(
        &parsed_node,
        NODE_NAME,
        &file_under_test,
        &config,
        &mut test_results,
    );

    assert_eq!(
        test_results.files_tested, expected_files_tested,
        "files_tested mismatch"
    );
    assert_eq!(
        test_results.files_failed, expected_files_failed,
        "files_failed mismatch"
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
