use colored::Colorize;
use godot_properties_parser::{parse_property_file, parse_scene_file};

use crate::{
    configuration::config::Config,
    reporting::{test_results::TestResults, warning::Warning},
    rules::{
        rule_allowed_file_location::execute_rule_allowed_file_location,
        rule_filename_snake_case::execute_rule_filename_snake_case,
        rule_parent_has_same_name::execute_rule_parent_has_same_name,
    },
    validation::{
        file_under_test::FileUnderTest, resource_validator::validate_resource_file,
        scene_validator::validate_scene_file,
    },
};

pub fn process_file(file_under_test: FileUnderTest, config: Config) -> Option<TestResults> {
    let mut test_results = TestResults::default();

    execute_rule_allowed_file_location(&file_under_test, &config, &mut test_results);
    execute_rule_filename_snake_case(&file_under_test, &config, &mut test_results);
    execute_rule_parent_has_same_name(&file_under_test, &config, &mut test_results);

    if file_under_test.extension == "tscn" || file_under_test.extension == "tres" {
        let file_content = match std::fs::read_to_string(&file_under_test.absolute_path) {
            Ok(content) => content,
            Err(_) => {
                test_results.warnings.push(Warning {
                    absolute_path: file_under_test.absolute_path.clone(),
                    message: String::from("Unable to read scene file"),
                });
                return None;
            }
        };
        match file_under_test.extension.as_str() {
            "tscn" => {
                match parse_scene_file(&file_content) {
                    Err(_warning) => {
                        test_results.warnings.push(Warning {
                            absolute_path: file_under_test.absolute_path.clone(),
                            message: String::from("Unable to parse scene file"),
                        });
                        return None;
                    }
                    Ok((_, scene_file)) => validate_scene_file(
                        scene_file,
                        &file_under_test,
                        &mut test_results,
                        &config,
                    ),
                };
            }
            "tres" => match parse_property_file(&file_content) {
                Err(_warning) => {
                    test_results.warnings.push(Warning {
                        absolute_path: file_under_test.absolute_path.clone(),
                        message: String::from("Unable to parse scene file"),
                    });
                    return None;
                }
                Ok((_, resource_file)) => validate_resource_file(
                    resource_file,
                    &file_under_test,
                    &mut test_results,
                    &config,
                ),
            },
            _ => (),
        }
    }

    if test_results.files_tested > 0
        && (test_results.files_failed > 0 || config.should_print_success)
    {
        let mut output = format!(
            "{} out of {} checks have failed for {} ...",
            test_results.files_failed,
            test_results.files_tested,
            file_under_test.relative_path.yellow()
        );

        for report in &test_results.failed_reports {
            output.push_str(&format!(
                "\n\t{} ({}):\n\t\t{}",
                "×".red(),
                report.rule_name.bright_black(),
                report.message
            ));
        }

        if config.should_print_success {
            for report in &test_results.successful_reports {
                output.push_str(&format!(
                    "\n\t{} ({}):\n\t\t{}",
                    "✓".green(),
                    report.rule_name.bright_black(),
                    report.message
                ));
            }
        }

        println!("{}\n", output);
    }

    Some(test_results)
}
