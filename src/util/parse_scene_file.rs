use std::collections::HashMap;

use colored::Colorize;

use crate::models::warning::Warning;

pub struct ParsedSceneFileData {
    pub meta: ParsedSceneEntry,
    pub nodes: Vec<ParsedSceneEntry>,
    pub external_resources: Vec<ParsedSceneEntry>,
    pub sub_resources: Vec<ParsedSceneEntry>,
    pub connections: Vec<ParsedSceneEntry>,
}

pub enum HeaderClassifier {
    GdScene,
    Node,
    ExtResource,
    SubResource,
    Connection,
}

pub struct ParsedSceneEntry {
    pub classifier: HeaderClassifier,
    pub header_properties: HashMap<String, String>,
    pub properties: HashMap<String, String>,
}

pub fn parse_scene_file(absolute_path: String) -> Result<ParsedSceneFileData, Warning> {
    let file_content = match std::fs::read_to_string(&absolute_path) {
        Ok(content) => content,
        Err(_) => {
            return Err(Warning {
                absolute_path,
                message: String::from("Unable to read scene file while parsing"),
            });
        }
    };
    let mut parsed_scene_file_data = ParsedSceneFileData {
        nodes: vec![],
        meta: ParsedSceneEntry {
            classifier: HeaderClassifier::GdScene,
            header_properties: HashMap::new(),
            properties: HashMap::new(),
        },
        external_resources: vec![],
        sub_resources: vec![],
        connections: vec![],
    };
    let mut last_scene_entry: Option<ParsedSceneEntry> = None;
    for line in file_content.lines() {
        if line.starts_with('[') && line.ends_with(']') {
            let parsed_scene_entry = match initialize_scene_entry_from_parsed_header(
                line.to_owned(),
            ) {
                Err(error) => {
                    return Err(Warning {
                        absolute_path,
                        message: error,
                    });
                }
                Ok(optional_scene) => {
                    if optional_scene.is_none() {
                        return Err(Warning {
                            absolute_path,
                            message: format!(
                                "{} (Line: {})",
                                "Unable to parse header value-pair as it contains invalid entries",
                                line
                            ),
                        });
                    }
                    optional_scene.unwrap()
                }
            };

            if let Some(entry) = last_scene_entry.take() {
                match entry.classifier {
                    HeaderClassifier::ExtResource => {
                        parsed_scene_file_data.external_resources.push(entry)
                    }
                    HeaderClassifier::GdScene => parsed_scene_file_data.meta = entry,
                    HeaderClassifier::Node => parsed_scene_file_data.nodes.push(entry),
                    HeaderClassifier::SubResource => {
                        parsed_scene_file_data.sub_resources.push(entry)
                    }
                    HeaderClassifier::Connection => parsed_scene_file_data.connections.push(entry),
                }
            }
            last_scene_entry = Some(parsed_scene_entry)
        } else {
            let property = parse_property(line.to_owned());
            if let (Some(prop), Some(ref mut entry)) = (property, last_scene_entry.as_mut()) {
                entry.properties.insert(prop.0, prop.1);
            }
        }
    }
    if let Some(entry) = last_scene_entry {
        parsed_scene_file_data.nodes.push(entry);
    }
    Ok(parsed_scene_file_data)
}

/**
 * TODO https://github.com/greenpixels/godot-arch/issues/4: This does not yet correctly handle multiline properties like:
 * ```
 * [sub_resource type="Curve2D" id="Curve2D_o3865"]
 *   _data = {
 *   "points": PackedVector2Array(0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 8)
 *   }
 *   point_count = 2
 * ```
 */
fn parse_property(property_content: String) -> Option<(String, String)> {
    return get_key_value_pair_from_string(&property_content)
        .map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned()));
}

fn get_key_value_pair_from_string(input: &str) -> Option<(&str, &str)> {
    let sep_index = input.find("=");
    if !sep_index.is_some() {
        return None;
    }
    let mut pair: (&str, &str) = input.split_at(sep_index.unwrap());
    pair.0 = pair.0.trim().trim_matches('"');
    pair.1 = pair
        .1
        .trim()
        .trim_start_matches("=\"")
        .trim_start_matches("= ")
        .trim_end_matches('"');
    return Some(pair);
}

fn initialize_scene_entry_from_parsed_header(
    header_content: String,
) -> Result<Option<ParsedSceneEntry>, String> {
    let mut header_entries = header_content
        .trim()
        .trim_start_matches("[")
        .trim_end_matches("]")
        .split(" ");

    // The first entry is always a classifier that does not conform to key=value
    let first_entry = header_entries.next();
    if first_entry.is_none() {
        return Ok(None);
    }

    let optional_result = initialize_scene_entry_with_classifier(first_entry.unwrap());
    if optional_result.is_none() {
        return Ok(None);
    }
    let mut result = optional_result.unwrap();

    for entry in header_entries {
        let optional_pair = get_key_value_pair_from_string(entry);
        if optional_pair.is_none() {
            continue;
        }
        let pair = optional_pair.unwrap();
        // TODO https://github.com/greenpixels/godot-arch/issues/5: find a clean way to handle built-in scripts
        if check_for_built_in_script(pair) {
            return Err(format!(
                "{}",
                "Error: Built-In Scripts are not supported and can't be properly parsed. Skipping affected file.".red(),
            ));
        }
        result
            .header_properties
            .insert(pair.0.to_owned(), pair.1.to_owned());
    }
    return Ok(Some(result));
}

fn check_for_built_in_script(key_value_pair: (&str, &str)) -> bool {
    return key_value_pair.0 == "type" && key_value_pair.1 == "\"GDScript\"";
}

fn initialize_scene_entry_with_classifier(entry: &str) -> Option<ParsedSceneEntry> {
    match entry {
        "gd_scene" => {
            return {
                Some(ParsedSceneEntry {
                    classifier: HeaderClassifier::GdScene,
                    header_properties: HashMap::new(),
                    properties: HashMap::new(),
                })
            };
        }
        "node" => {
            return {
                Some(ParsedSceneEntry {
                    classifier: HeaderClassifier::Node,
                    header_properties: HashMap::new(),
                    properties: HashMap::new(),
                })
            };
        }
        "ext_resource" => {
            return {
                Some(ParsedSceneEntry {
                    classifier: HeaderClassifier::ExtResource,
                    header_properties: HashMap::new(),
                    properties: HashMap::new(),
                })
            };
        }
        "sub_resource" => {
            return {
                Some(ParsedSceneEntry {
                    classifier: HeaderClassifier::SubResource,
                    header_properties: HashMap::new(),
                    properties: HashMap::new(),
                })
            };
        }
        "connection" => {
            return {
                Some(ParsedSceneEntry {
                    classifier: HeaderClassifier::Connection,
                    header_properties: HashMap::new(),
                    properties: HashMap::new(),
                })
            };
        }
        _ => return None,
    }
}
