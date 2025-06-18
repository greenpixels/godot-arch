use colored::Colorize;
use glob_match::glob_match;
use std::collections::HashMap;
use std::fs::{DirEntry, exists};
use std::io;
use std::path::Path;
mod models;
mod rules;
mod util;
use crate::models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults};
use crate::rules::rule_allowed_file_location::execute_rule_allowed_file_location;
use crate::rules::rule_filename_snake_case::execute_rule_filename_snake_case;
use crate::rules::rule_parent_has_same_name::execute_rule_parent_has_same_name;
use crate::rules::rule_root_node_is_file_name_pascal::rule_root_node_is_file_name_pascal;
use crate::rules::rule_scene_nodes_pascal_case::execute_rule_scene_needs_pascal_case;
use crate::util::{ansi::enable_ansi_support, visit_dirs::visit_dirs};

lazy_static::lazy_static! {
    static ref TEST_RESULTS: std::sync::Mutex<TestResults> = std::sync::Mutex::new(
         TestResults { files_tested: 0, files_failed: 0 }
    );

    static ref CONFIG: Config = {
        Config {
            project_path: String::from("./"),
            wait_for_input_before_close: false,
            allowed_file_locations: HashMap::new(),
            ignore_patterns: models::config::IgnorePatterns {
                overall: Vec::new(),
                allowed_file_location: Vec::new(),
                filename_snake_case: Vec::new(),
                parent_has_same_name: Vec::new(),
                scene_nodes_pascal_case: Vec::new(),
                root_node_is_file_name_pascal: Vec::new(),
            },
            node_name_pascal_case_exceptions: Vec::new(),
            allow_screaming_snake_case_in_node_names: true,
            should_print_success: true
        }
    };

    static ref UPPERCASE_TO_PASCAL_CASE: HashMap<String, String> = {
        let mut m = HashMap::new();
        for rewrite in &CONFIG.node_name_pascal_case_exceptions {
            for (k, v) in rewrite {
                m.insert(k.clone(), v.clone());
            }
        }
        m
    };    static ref IGNORE_PATTERNS: Vec<String> = {
        CONFIG.ignore_patterns.overall.clone()
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();
    let start_time = std::time::Instant::now();
    let path_string: &str = CONFIG.project_path.as_str();
    let path = Path::new(path_string);
    let final_test_results = TEST_RESULTS.lock().unwrap();
    if !exists(path).is_ok() {
        panic!("Tried to index a path that does not exist {path_string}")
    }
    println!("Indexing in {path_string}");
    visit_dirs(path_string, &IGNORE_PATTERNS, path, &handle_file)
        .is_err()
        .then(|| "Something went wrong");
    println!(
        "\n>\t{} tests of {} total have failed",
        final_test_results.files_failed, final_test_results.files_tested
    );

    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:.2?}", elapsed_time);
    if CONFIG.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    if final_test_results.files_failed != 0 {
        return Err("Some tests were not succesful".into());
    }
    return Ok(());
}

fn handle_file(input_path_string: &str, entry: &DirEntry) {
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

    validate_file(file_under_test, extension)
}

fn validate_file(file: FileUnderTest, extension: &str) {
    if should_skip_file(&file) {
        return;
    }
    println!(
        "\n\n>> Testing {} in {}...",
        file.file_name.yellow(),
        file.relative_path.yellow()
    );
    execute_rule_allowed_file_location(&file, &CONFIG, &mut TEST_RESULTS.lock().unwrap());
    execute_rule_filename_snake_case(&file, &CONFIG, &mut TEST_RESULTS.lock().unwrap());
    execute_rule_parent_has_same_name(&file, &CONFIG, &mut TEST_RESULTS.lock().unwrap());
    execute_rule_parent_has_same_name(&file, &CONFIG, &mut TEST_RESULTS.lock().unwrap());
    if extension == "tscn" {
        validate_scene_nodes(file);
    }
}

fn normalize_path(path: &str) -> String {
    format!(
        "./{}",
        path.replace('\\', "/").trim_start_matches('/').to_owned()
    )
}

fn should_skip_file(file: &FileUnderTest) -> bool {
    IGNORE_PATTERNS
        .iter()
        .any(|pattern| glob_match(pattern, &file.relative_path))
}

fn validate_scene_nodes(file: FileUnderTest) {
    let mut should_check_scene_nodes_pascal_case = true;
    let mut should_check_root_node_is_file_name_pascal = true;
    for pattern in CONFIG.ignore_patterns.scene_nodes_pascal_case.iter() {
        if glob_match(pattern, &file.relative_path) {
            should_check_scene_nodes_pascal_case = false;
        }
    }
    for pattern in CONFIG.ignore_patterns.root_node_is_file_name_pascal.iter() {
        if glob_match(pattern, &file.relative_path) {
            should_check_root_node_is_file_name_pascal = false;
        }
    }

    if !should_check_scene_nodes_pascal_case && !should_check_root_node_is_file_name_pascal {
        return;
    }

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
            if is_root_node && should_check_root_node_is_file_name_pascal {
                is_root_node = false;
                rule_root_node_is_file_name_pascal(
                    node_name.as_str(),
                    &file,
                    &CONFIG,
                    &mut TEST_RESULTS.lock().unwrap(),
                );
            }
            if should_check_scene_nodes_pascal_case {
                execute_rule_scene_needs_pascal_case(
                    node_name.as_str(),
                    &file,
                    &CONFIG,
                    &mut TEST_RESULTS.lock().unwrap(),
                );
            }
        }
    }
}
