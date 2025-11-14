use crate::{
    rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder,
    tests::mocks::{
        get_config_mock, get_file_under_test_mock, get_parsed_scene_file_data_mock,
        get_scene_external_resource, get_scene_node_mock_with_external_script,
        get_test_results_mock,
    },
};

#[test]
fn test_rule_should_pass_with_script_in_same_location() {
    const TEST_ID: &str = "myverycooltestid";
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    let mut parsed_scene = get_parsed_scene_file_data_mock();

    parsed_scene
        .nodes
        .push(get_scene_node_mock_with_external_script("File", TEST_ID));

    parsed_scene
        .external_resources
        .push(get_scene_external_resource(
            "res://resources/my_folder/file.gd",
            TEST_ID,
        ));

    execute_rule_root_node_script_in_same_folder(
        &parsed_scene,
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_rule_should_fail_with_script_in_different_location() {
    const TEST_ID: &str = "myverycooltestid";
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    let mut parsed_scene = get_parsed_scene_file_data_mock();

    parsed_scene
        .nodes
        .push(get_scene_node_mock_with_external_script("File", TEST_ID));

    parsed_scene
        .external_resources
        .push(get_scene_external_resource(
            "res://addons/cool_script/file.gd",
            TEST_ID,
        ));

    execute_rule_root_node_script_in_same_folder(
        &parsed_scene,
        &file_under_test,
        &config,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 1);
}
