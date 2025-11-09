use colored::Colorize;
use std::fs::{DirEntry, exists};
use std::path::Path;
use std::{env, io};
mod models;
mod rules;
#[cfg(test)]
mod tests;
mod util;

use crate::models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults};
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

lazy_static::lazy_static! {
    static ref CONFIG: Config = {
        let args: Vec<String> = env::args().collect();
        let configuration_path: &str;
        if args.len() > 1 {
            configuration_path = &args[1];
        } else {
            configuration_path = "godot-arch.config.yaml";
        }
        let config_content = std::fs::read_to_string(configuration_path)
            .expect("Failed to read config file");
        serde_yaml::from_str(&config_content)
            .expect("Failed to parse config file")
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();
    let start_time = std::time::Instant::now();
    let path_string: &str = CONFIG.project_path.as_str();
    let path = Path::new(path_string);
    let mut test_results: TestResults = TestResults {
        files_tested: 0,
        files_failed: 0,
    };

    if !exists(path).is_ok() {
        panic!("Tried to index a path that does not exist {path_string}")
    }
    println!("Indexing in {path_string}");

    visit_dirs(path_string, &CONFIG, path, &mut test_results, &handle_file)
        .is_err()
        .then(|| println!("Something went wrong!"));

    println!(
        "\n>\t{} tests of {} total have failed",
        test_results.files_failed, test_results.files_tested
    );

    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:.2?}", elapsed_time);
    if CONFIG.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    if test_results.files_failed != 0 {
        return Err("Some tests were not succesful".into());
    }
    return Ok(());
}

fn handle_file(input_path_string: &str, entry: &DirEntry, mut test_results: &mut TestResults) {
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

    execute_rule_allowed_file_location(&file_under_test, &CONFIG, &mut test_results);
    execute_rule_filename_snake_case(&file_under_test, &CONFIG, &mut test_results);
    execute_rule_parent_has_same_name(&file_under_test, &CONFIG, &mut test_results);

    if extension == "tscn" {
        validate_scene_nodes(&file_under_test, test_results);
    }

    if previous_tests != test_results.files_tested
        && (test_results.files_failed > previous_fails || CONFIG.should_print_success)
    {
        println!(
            ">>>\t{} errors in {} ...\n\n",
            test_results.files_failed - previous_fails,
            file_under_test.relative_path.yellow()
        );
    }
}

fn validate_scene_nodes(file: &FileUnderTest, test_results: &mut TestResults) {
    let file_content = match std::fs::read_to_string(&file.absolute_path) {
        Ok(content) => content,
        Err(_) => return,
    };

    let mut is_root_node = true;
    for line in file_content.lines() {
        if !line.starts_with('[') || !line.ends_with(']') {
            continue;
        }

        let trimmed_line = line.trim_matches(|c| c == '[' || c == ']');
        let (section_key, rest) = match trimmed_line.split_once(' ') {
            Some(parts) => parts,
            None => continue,
        };

        if section_key != "node" {
            continue;
        }

        for pair in rest.split_whitespace() {
            let (_key, value) = match pair.split_once('=') {
                Some(kv) if kv.0 == "name" => kv,
                _ => continue,
            };
            let node_name = value.replace("\"", "");
            if is_root_node {
                is_root_node = false;
                execute_rule_root_node_is_file_name_pascal(
                    node_name.as_str(),
                    file,
                    &CONFIG,
                    test_results,
                );
            }
            execute_rule_scene_needs_pascal_case(node_name.as_str(), file, &CONFIG, test_results);
        }
    }
}
