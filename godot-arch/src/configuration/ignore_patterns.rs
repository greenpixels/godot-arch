use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct IgnorePatterns {
    #[serde(default)]
    pub overall: Vec<String>,
    #[serde(rename = "rule-allowed-file-location")]
    #[serde(default)]
    pub allowed_file_location: Vec<String>,
    #[serde(rename = "rule-allowed-custom-resource-location")]
    #[serde(default)]
    pub allowed_custom_resource_location: Vec<String>,
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
