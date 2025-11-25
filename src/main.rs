use clap::{Parser, arg};
use colored::Colorize;
use godot_properties_parser::parse_scene_file;
use std::fs::{DirEntry, exists};
use std::path::Path;
use std::{io, vec};
mod models;
mod rules;
#[cfg(test)]
mod tests;
mod util;

use crate::models::warning::Warning;
use crate::models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults};
use crate::rules::rule_node_depth_fits_max_depth::execute_rule_node_depth_fits_max_depth;
use crate::rules::rule_root_node_script_in_same_folder::execute_rule_root_node_script_in_same_folder;
use crate::rules::{
    rule_allowed_file_location::execute_rule_allowed_file_location,
    rule_filename_snake_case::execute_rule_filename_snake_case,
    rule_parent_has_same_name::execute_rule_parent_has_same_name,
    rule_root_node_is_file_name_pascal::execute_rule_root_node_is_file_name_pascal,
    rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case,
};

use crate::util::{
    ansi::enable_ansi_support, normalize_path::normalize_path, visit_dirs::visit_dirs,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to godot project root
    #[arg(short = 'p', long = "project", default_value_t = String::from("./"))]
    project_path: String,

    /// Path to configuration file
    #[arg(
        short = 'c',
        long = "config",
        default_value_t = String::from("./godot-arch.config.yaml")
    )]
    config_path: String,

    /// Location in which to save a report in
    #[arg(long = "report", default_value_t = String::from("./"))]
    report_location: String,
}

fn load_config(path: String) -> Config {
    let config_content = std::fs::read_to_string(path).expect("Failed to read config file");
    serde_yaml::from_str(&config_content).expect("Failed to parse config file")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();
    let Args {
        config_path,
        project_path,
        report_location,
    } = Args::parse();
    let config: Config = load_config(config_path);
    let start_time = std::time::Instant::now();
    let path_string: &str = &project_path;
    let path = Path::new(path_string);
    let mut test_results: TestResults = TestResults {
        files_tested: 0,
        files_failed: 0,
        warnings: vec![],
        failed_reports: vec![],
        successful_reports: vec![],
    };

    if exists(path).is_err() {
        panic!("Tried to index a path that does not exist {path_string}")
    }
    println!("Indexing in {path_string}");

    visit_dirs(path_string, &config, path, &mut test_results, &handle_file)
        .is_err()
        .then(|| println!("Something went wrong!"));

    if !test_results.warnings.is_empty() {
        for warning in &test_results.warnings {
            println!(
                "{} {}\n>>>     in {}",
                "Warning:".yellow(),
                warning.message.yellow(),
                warning.absolute_path.bright_black()
            );
        }
    }

    println!(
        "\n>\t{} tests of {} total have failed",
        test_results.files_failed, test_results.files_tested
    );

    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:.2?}", elapsed_time);
    if config.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    let report_path = Path::new(&report_location).join("godot-arch-report.json");
    println!("{}{}", "Writing report to ", report_path.display());
    match serde_json::to_string_pretty(&test_results) {
        Ok(report_json) => {
            if let Some(parent) = Path::new(&report_path).parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Failed to create report directory: {}", e);
                }
            }
            if let Err(e) = std::fs::write(&report_path, report_json) {
                eprintln!("Failed to write report file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize report: {}", e);
        }
    }

    if test_results.files_failed != 0 {
        return Err("Some tests were not successful".into());
    }

    Ok(())
}

fn handle_file(
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

    if extension == "tscn" {
        validate_scene_nodes(&file_under_test, test_results, config);
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

fn validate_scene_nodes(file: &FileUnderTest, test_results: &mut TestResults, config: &Config) {
    let file_content = match std::fs::read_to_string(&file.absolute_path) {
        Ok(content) => content,
        Err(_) => {
            test_results.warnings.push(Warning {
                absolute_path: file.absolute_path.clone(),
                message: String::from("Unable to read scene file"),
            });
            return;
        }
    };
    let parsed_scene_file = match parse_scene_file(&file_content) {
        Err(_warning) => {
            test_results.warnings.push(Warning {
                absolute_path: file.absolute_path.clone(),
                message: String::from("Unable to parse scene file"),
            });
            return;
        }
        Ok((_, scene_file)) => scene_file,
    };

    let mut is_root_node = true;
    execute_rule_root_node_script_in_same_folder(&parsed_scene_file, file, config, test_results);
    for node in &parsed_scene_file.nodes {
        let node_name = match node.properties.iter().find(|p| p.key == "name") {
            None => return,
            Some(prop) => &prop.value,
        };

        if is_root_node {
            execute_rule_root_node_is_file_name_pascal(
                node_name.as_str(),
                file,
                config,
                test_results,
            );
            is_root_node = false;
        }
        execute_rule_scene_needs_pascal_case(node_name.as_str(), file, config, test_results);
        execute_rule_node_depth_fits_max_depth(node, node_name, file, config, test_results);
    }
}
