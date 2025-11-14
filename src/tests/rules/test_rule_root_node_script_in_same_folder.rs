use crate::{
    rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder,
    tests::mocks::{
        get_config_mock, get_file_under_test_mock, get_parsed_scene_file_data_mock,
        get_scene_external_resource, get_scene_node_mock_with_external_script,
        get_test_results_mock,
    },
};

fn assert_results(script_path: &str, expected_files_tested: i32, expected_files_failed: i32) {
    let mut test_results = get_test_results_mock();
    let file_under_test = get_file_under_test_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    let mut parsed_scene = get_parsed_scene_file_data_mock();

    parsed_scene
        .nodes
        .push(get_scene_node_mock_with_external_script(
            "File",
            "myverycooltestid",
        ));

    parsed_scene
        .external_resources
        .push(get_scene_external_resource(script_path, "myverycooltestid"));

    execute_rule_root_node_script_in_same_folder(
        &parsed_scene,
        &file_under_test,
        &config,
        &mut test_results,
    );

    assert_eq!(test_results.files_tested, expected_files_tested);
    assert_eq!(test_results.files_failed, expected_files_failed);
}

#[test]
fn test_rule_should_pass_with_script_in_same_location() {
    assert_results("res://resources/my_folder/file.gd", 1, 0);
}

#[test]
fn test_rule_should_fail_with_script_in_different_location() {
    assert_results("res://addons/cool_script/file.gd", 1, 1);
}
