use std::collections::HashMap;

use serde::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[serde(rename = "ignorePatterns")]
    pub ignore_patterns: IgnorePatterns,
    #[serde(default)]
    #[serde(rename = "allowedFileLocations")]
    pub allowed_file_locations: HashMap<String, Vec<String>>,
    #[serde(default)]
    #[serde(rename = "nodeNamePascalCaseExceptions")]
    pub node_name_pascal_case_exceptions: Vec<HashMap<String, String>>,
    #[serde(default)]
    #[serde(rename = "allowScreamingSnakeCaseInNodeNames")]
    pub allow_screaming_snake_case_in_node_names: bool,
    #[serde(default)]
    #[serde(rename = "shouldPrintSuccess")]
    pub should_print_success: bool,
    #[serde(default)]
    #[serde(rename = "projectPath")]
    pub project_path: String,
    #[serde(rename = "waitForInputBeforeClose")]
    #[serde(default)]
    pub wait_for_input_before_close: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct IgnorePatterns {
    #[serde(default)]
    pub overall: Vec<String>,
    #[serde(rename = "rule-allowed-file-location")]
    #[serde(default)]
    pub allowed_file_location: Vec<String>,
    #[serde(rename = "rule-filename-snake-case")]
    #[serde(default)]
    pub filename_snake_case: Vec<String>,
    #[serde(rename = "rule-parent-has-same-name")]
    #[serde(default)]
    pub parent_has_same_name: Vec<String>,
    #[serde(rename = "rule-scene-nodes-pascal-case")]
    #[serde(default)]
    pub scene_nodes_pascal_case: Vec<String>,
    #[serde(rename = "rule-root-node-is-file-name-pascal")]
    #[serde(default)]
    pub root_node_is_file_name_pascal: Vec<String>,
}
