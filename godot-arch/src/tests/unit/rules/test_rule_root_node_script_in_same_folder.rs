use crate::{
    rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder,
    tests::mocks::{
        get_config_mock, get_file_under_check_mock, get_parsed_scene_file_data_mock,
        get_scene_external_resource, get_scene_node_mock_with_external_script,
        get_check_results_mock,
    },
};

fn assert_results(script_path: &str, expected_files_checked: i32, expected_checks_failed: i32) {
    let mut check_results = get_check_results_mock();
    let file_under_check = get_file_under_check_mock("resources/my_folder", "file", "tscn");
    let config = get_config_mock();
    let mut parsed_scene = get_parsed_scene_file_data_mock();

    parsed_scene
        .nodes
        .push(get_scene_node_mock_with_external_script(
            "File",
            "myverycooltestid",
        ));

    parsed_scene
        .ext_resources
        .push(get_scene_external_resource(script_path, "myverycooltestid"));

    execute_rule_root_node_script_in_same_folder(
        &parsed_scene,
        &file_under_check,
        &config,
        &mut check_results,
    );

    assert_eq!(check_results.files_checked, expected_files_checked);
    assert_eq!(check_results.files_failed, expected_checks_failed);
}

#[test]
fn test_rule_should_pass_with_script_in_same_location() {
    assert_results("res://resources/my_folder/file.gd", 1, 0);
}

#[test]
fn test_rule_should_fail_with_script_in_different_location() {
    assert_results("res://addons/cool_script/file.gd", 1, 1);
}
