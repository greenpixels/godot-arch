use std::collections::HashMap;

use serde::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[serde(rename = "ignorePatterns")]
    pub ignore_patterns: IgnorePatterns,
    #[serde(default)]
    #[serde(rename = "includePatterns")]
    pub include_patterns: IncludePatterns,
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
    #[serde(rename = "waitForInputBeforeClose")]
    #[serde(default)]
    pub wait_for_input_before_close: bool,
    #[serde(rename = "maxNodeDepth")]
    #[serde(default = "default_max_node_depth")]
    pub max_node_depth: usize,
}

fn default_max_node_depth() -> usize {
    4
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
    #[serde(rename = "rule-root-node-script-in-same-folder")]
    #[serde(default)]
    pub root_node_script_in_same_folder: Vec<String>,
    #[serde(rename = "rule-node-depth-fits-max-depth")]
    #[serde(default)]
    pub node_depth_fits_max_depth: Vec<String>,
}
#[derive(Debug, Deserialize, Default)]
pub struct IncludePatterns {
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
    #[serde(rename = "rule-root-node-script-in-same-folder")]
    #[serde(default)]
    pub root_node_script_in_same_folder: Vec<String>,
    #[serde(rename = "rule-node-depth-fits-max-depth")]
    #[serde(default)]
    pub node_depth_fits_max_depth: Vec<String>,
}
