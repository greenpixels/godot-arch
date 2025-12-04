use colored::Colorize;
use godot_properties_parser::{parse_property_file, parse_scene_file};
use std::fs::DirEntry;

use crate::{
    configuration::config::Config,
    reporting::{test_results::TestResults, warning::Warning},
    rules::{
        rule_allowed_file_location::execute_rule_allowed_file_location,
        rule_filename_snake_case::execute_rule_filename_snake_case,
        rule_parent_has_same_name::execute_rule_parent_has_same_name,
    },
    util::normalize_path::normalize_path,
    validation::{
        file_under_test::FileUnderTest, resource_validator::validate_resource_file,
        scene_validator::validate_scene_file,
    },
};

pub fn process_file(
    input_path_string: &str,
    entry: &DirEntry,
    test_results: &mut TestResults,
    config: &Config,
) {
    let path = entry.path();
    let full_path = path.to_str().unwrap_or("");
    let file_name = path
        .file_name()
        .and_then(|comp| comp.to_str())
        .unwrap_or("");
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let file_under_test = FileUnderTest {
        file_name: file_name.to_owned(),
        path: path.to_owned(),
        extension: extension.to_owned(),
        absolute_path: path
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_owned(),
        relative_path: normalize_path(
            full_path
                .strip_prefix(input_path_string)
                .unwrap_or(file_name),
        ),
    };

    let previous_fails = test_results.files_failed;
    let previous_tests = test_results.files_tested;

    execute_rule_allowed_file_location(&file_under_test, config, test_results);
    execute_rule_filename_snake_case(&file_under_test, config, test_results);
    execute_rule_parent_has_same_name(&file_under_test, config, test_results);

    if extension == "tscn" || extension == "tres" {
        let file_content = match std::fs::read_to_string(&file_under_test.absolute_path) {
            Ok(content) => content,
            Err(_) => {
                test_results.warnings.push(Warning {
                    absolute_path: file_under_test.absolute_path.clone(),
                    message: String::from("Unable to read scene file"),
                });
                return;
            }
        };
        match extension {
            "tscn" => {
                match parse_scene_file(&file_content) {
                    Err(_warning) => {
                        test_results.warnings.push(Warning {
                            absolute_path: file_under_test.absolute_path.clone(),
                            message: String::from("Unable to parse scene file"),
                        });
                        return;
                    }
                    Ok((_, scene_file)) => {
                        validate_scene_file(scene_file, &file_under_test, test_results, config)
                    }
                };
            }
            "tres" => match parse_property_file(&file_content) {
                Err(_warning) => {
                    test_results.warnings.push(Warning {
                        absolute_path: file_under_test.absolute_path.clone(),
                        message: String::from("Unable to parse scene file"),
                    });
                    return;
                }
                Ok((_, resource_file)) => {
                    validate_resource_file(resource_file, &file_under_test, test_results, config)
                }
            },
            _ => (),
        }
    }

    if previous_tests != test_results.files_tested
        && (test_results.files_failed > previous_fails || config.should_print_success)
    {
        println!(
            ">>>\t{} errors in {} ...\n\n",
            test_results.files_failed - previous_fails,
            file_under_test.relative_path.yellow()
        );
    }
}
