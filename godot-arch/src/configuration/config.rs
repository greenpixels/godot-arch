use std::collections::HashMap;

use serde::Deserialize;

use crate::configuration::{ignore_patterns::IgnorePatterns, include_patterns::IncludePatterns};

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "ignorePatterns")]
    pub ignore_patterns: IgnorePatterns,
    #[serde(rename = "includePatterns")]
    pub include_patterns: IncludePatterns,
    #[serde(rename = "allowedFileLocations")]
    pub allowed_file_locations: HashMap<String, Vec<String>>,
    #[serde(rename = "allowedCustomResourceLocations")]
    pub allowed_custom_resource_locations: HashMap<String, Vec<String>>,
    #[serde(rename = "nodeNamePascalCaseExceptions")]
    pub node_name_pascal_case_exceptions: Vec<HashMap<String, String>>,
    #[serde(rename = "allowScreamingSnakeCaseInNodeNames")]
    pub allow_screaming_snake_case_in_node_names: bool,
    #[serde(rename = "shouldPrintSuccess")]
    pub should_print_success: bool,
    #[serde(rename = "waitForInputBeforeClose")]
    pub wait_for_input_before_close: bool,
    #[serde(rename = "maxNodeDepth")]
    pub max_node_depth: usize,
    #[serde(rename = "failUnmatchedCustomResources")]
    pub should_fail_unmatched_custom_resources: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignore_patterns: IgnorePatterns::default(),
            include_patterns: IncludePatterns::default(),
            allowed_file_locations: HashMap::new(),
            allowed_custom_resource_locations: HashMap::new(),
            node_name_pascal_case_exceptions: vec![],
            allow_screaming_snake_case_in_node_names: false,
            should_print_success: false,
            wait_for_input_before_close: false,
            max_node_depth: 4,
            should_fail_unmatched_custom_resources: false,
        }
    }
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}
