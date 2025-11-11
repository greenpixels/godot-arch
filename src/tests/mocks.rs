use std::{collections::HashMap, path::PathBuf, vec};

use crate::models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults};

pub fn get_test_results_mock() -> TestResults {
    return TestResults {
        files_tested: 0,
        files_failed: 0,
        warnings: vec![],
    };
}

pub fn get_file_under_test_mock(
    path_in_godot: &str,
    file_base_name: &str,
    extension: &str,
) -> FileUnderTest {
    return FileUnderTest {
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
    };
}

pub fn get_config_mock() -> Config {
    return Config {
        allow_screaming_snake_case_in_node_names: false,
        allowed_file_locations: HashMap::new(),
        ignore_patterns: crate::models::config::IgnorePatterns {
            ..Default::default()
        },
        include_patterns: crate::models::config::IncludePatterns {
            filename_snake_case: vec!["./**".to_owned()],
            parent_has_same_name: vec!["./**".to_owned()],
            scene_nodes_pascal_case: vec!["./**".to_owned()],
            root_node_is_file_name_pascal: vec!["./**".to_owned()],
            root_node_script_in_same_folder: vec!["./**".to_owned()],
            node_depth_fits_max_depth: vec!["./**".to_owned()],
        },
        node_name_pascal_case_exceptions: vec![],
        project_path: ".".to_string(),
        should_print_success: true,
        wait_for_input_before_close: false,
        max_node_depth: 4,
    };
}
