use crate::{
    rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth,
    tests::mocks::{
        get_config_mock, get_file_under_test_mock, get_scene_node_mock_with_external_script,
        get_test_results_mock,
    },
};

#[test]
fn test_rule_should_pass_with_node_having_no_parent() {
    const TEST_ID: &str = "myverycooltestid";
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config.max_node_depth = 3;
    let mut parsed_node = get_scene_node_mock_with_external_script("File", TEST_ID);
    parsed_node
        .header_properties
        .insert(String::from("parent"), String::from("."));

    execute_rule_node_depth_fits_max_depth(
        &parsed_node,
        "NodeName",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_pass_with_node_depth_less_than_max() {
    const TEST_ID: &str = "myverycooltestid";
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config.max_node_depth = 3;

    let mut parsed_node = get_scene_node_mock_with_external_script("File", TEST_ID);
    parsed_node.header_properties.insert(
        String::from("parent"),
        String::from("HBoxContainer/Control"),
    );

    execute_rule_node_depth_fits_max_depth(
        &parsed_node,
        "NodeName",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_with_node_depth_more_than_max() {
    const TEST_ID: &str = "myverycooltestid";
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let mut config = get_config_mock();
    config.max_node_depth = 3;

    let mut parsed_node = get_scene_node_mock_with_external_script("File", TEST_ID);
    parsed_node.header_properties.insert(
        String::from("parent"),
        String::from("HBoxContainer/Control/Node2D/CPUParticle2D/Control"),
    );

    execute_rule_node_depth_fits_max_depth(
        &parsed_node,
        "NodeName",
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
