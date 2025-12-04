use std::collections::HashMap;
use std::{path::PathBuf, vec};

use crate::configuration::config::Config;
use crate::configuration::ignore_patterns::IgnorePatterns;
use crate::configuration::include_patterns::IncludePatterns;
use crate::reporting::test_results::TestResults;
use crate::validation::file_under_test::FileUnderTest;
use godot_properties_parser::parsers::{
    parser_property::UntypedProperty, parser_property_file::Section, parser_scene_file::SceneFile,
};

pub fn get_test_results_mock() -> TestResults {
    TestResults::default()
}

pub fn get_file_under_test_mock(
    path_in_godot: &str,
    file_base_name: &str,
    extension: &str,
) -> FileUnderTest {
    FileUnderTest {
        path: PathBuf::from(format!(
            "./{}/{}.{}",
            path_in_godot, file_base_name, extension
        )),
        absolute_path: format!(
            "{}/{}/{}.{}",
            "/home/username/projects/godot/gamename", path_in_godot, file_base_name, extension
        ),
        extension: extension.to_owned(),
        file_name: format!("{}.{}", file_base_name, extension),
        relative_path: format!("./{}/{}.{}", path_in_godot, file_base_name, extension),
    }
}

pub fn get_config_mock() -> Config {
    Config {
        allow_screaming_snake_case_in_node_names: false,
        allowed_file_locations: HashMap::new(),
        ignore_patterns: IgnorePatterns {
            ..Default::default()
        },
        include_patterns: IncludePatterns {
            filename_snake_case: vec!["./**".to_owned()],
            parent_has_same_name: vec!["./**".to_owned()],
            scene_nodes_pascal_case: vec!["./**".to_owned()],
            root_node_is_file_name_pascal: vec!["./**".to_owned()],
            root_node_script_in_same_folder: vec!["./**".to_owned()],
            node_depth_fits_max_depth: vec!["./**".to_owned()],
        },
        node_name_pascal_case_exceptions: vec![],
        should_print_success: true,
        wait_for_input_before_close: false,
        max_node_depth: 4,
        allowed_custom_resource_locations: HashMap::new(),
        should_fail_unmatched_custom_resources: false,
    }
}

pub fn get_parsed_scene_file_data_mock() -> SceneFile {
    SceneFile::new()
}

pub fn get_scene_node_mock_with_external_script(name: &str, script_id: &str) -> Section {
    let properties = vec![
        UntypedProperty {
            key: String::from("name"),
            value: String::from(name),
        },
        UntypedProperty {
            key: String::from("type"),
            value: String::from("Node2D"),
        },
        UntypedProperty {
            key: String::from("script"),
            value: format!("ExtResource(\"{}\")", script_id),
        },
    ];

    Section {
        header_type: String::from("node"),
        properties,
    }
}

pub fn get_scene_external_resource(res_path: &str, script_id: &str) -> Section {
    let properties = vec![
        UntypedProperty {
            key: String::from("type"),
            value: String::from("Script"),
        },
        UntypedProperty {
            key: String::from("path"),
            value: String::from(res_path),
        },
        UntypedProperty {
            key: String::from("id"),
            value: String::from(script_id),
        },
        UntypedProperty {
            key: String::from("uid"),
            value: String::from("uid://b20d51xcgous8"),
        },
    ];

    Section {
        header_type: String::from("ext_resource"),
        properties,
    }
}
