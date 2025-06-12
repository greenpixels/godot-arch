use colored::Colorize;
use glob_match::glob_match;
use inflections::Inflect;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{DirEntry, exists, read_dir};
use std::path::{Path, PathBuf};
use std::{env, io};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    #[serde(rename = "ignorePatterns")]
    ignore_patterns: IgnorePatterns,
    #[serde(default)]
    #[serde(rename = "allowedFileLocations")]
    allowed_file_locations: HashMap<String, Vec<String>>,
    #[serde(default)]
    #[serde(rename = "nodeNamePascalCaseExceptions")]
    node_name_pascal_case_exceptions: Vec<HashMap<String, String>>,
    #[serde(default)]
    #[serde(rename = "allowScreamingSnakeCaseInNodeNames")]
    allow_screaming_snake_case_in_node_names: bool,
    #[serde(default)]
    #[serde(rename = "shouldPrintSuccess")]
    should_print_success: bool,
    #[serde(default)]
    #[serde(rename = "projectPath")]
    project_path: String,
    #[serde(rename = "waitForInputBeforeClose")]
    #[serde(default)]
    wait_for_input_before_close: bool,
}

#[derive(Debug, Deserialize, Default)]
struct IgnorePatterns {
    #[serde(default)]
    overall: Vec<String>,
    #[serde(rename = "rule-allowed-file-location")]
    #[serde(default)]
    allowed_file_location: Vec<String>,
    #[serde(rename = "rule-filename-snake-case")]
    #[serde(default)]
    filename_snake_case: Vec<String>,
    #[serde(rename = "rule-parent-has-same-name")]
    #[serde(default)]
    parent_has_same_name: Vec<String>,
    #[serde(rename = "rule-scene-nodes-pascal-case")]
    #[serde(default)]
    scene_nodes_pascal_case: Vec<String>,
    #[serde(rename = "rule-root-node-is-file-name-pascal")]
    #[serde(default)]
    root_node_is_file_name_pascal: Vec<String>,
}

static mut VALIDATION_COUNT: i32 = 0;
static mut VALIDATION_FAILS: i32 = 0;

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

struct FileUnderTest {
    absolute_path: String,
    relative_path: String,
    file_name: String,
    path: PathBuf,
    extension: String,
}

fn enable_ansi_support() {
    #[cfg(windows)]
    {
        use std::ptr::null_mut;
        use winapi::um::consoleapi::GetConsoleMode;
        use winapi::um::consoleapi::SetConsoleMode;
        use winapi::um::processenv::GetStdHandle;
        use winapi::um::winbase::STD_OUTPUT_HANDLE;
        use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle != null_mut() {
                let mut mode = 0;
                if GetConsoleMode(handle, &mut mode) != 0 {
                    SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();
    let start_time = std::time::Instant::now();

    let path_string: &str = CONFIG.project_path.as_str();
    let path = Path::new(path_string);

    if !exists(path).is_ok() {
        panic!("Tried to index a path that does not exist {path_string}")
    }
    println!("Indexing in {path_string}");
    visit_dirs(path_string, path, &handle_file)
        .is_err()
        .then(|| "Something went wrong");
    println!(
        "\n>\t{} tests of {} total have failed",
        unsafe { VALIDATION_FAILS },
        unsafe { VALIDATION_COUNT }
    );

    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:.2?}", elapsed_time);
    if CONFIG.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    if unsafe { VALIDATION_FAILS != 0 } {
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
    validate_file_root(&file);
    validate_name(&file);
    match extension {
        "tscn" => {
            validate_parent_has_same_name(&file);
            validate_scene_nodes(file);
        }
        "tres" => {
            validate_parent_has_same_name(&file);
        }
        "gd" => {
            validate_parent_has_same_name(&file);
        }
        _ => return,
    }
}

fn validate_parent_has_same_name(file: &FileUnderTest) {
    // Check if this file should be skipped for parent name validation
    for pattern in CONFIG.ignore_patterns.parent_has_same_name.iter() {
        if glob_match(pattern, &file.relative_path) {
            return;
        }
    }

    let parent_option = file.path.parent();
    let mut has_parent_with_same_name = false;
    if parent_option.is_some() {
        let parent = parent_option.unwrap();
        let parent_file_name_option = parent.file_name();
        if parent_file_name_option.is_some() {
            let file_name = parent_file_name_option.unwrap().to_str().unwrap_or("");
            has_parent_with_same_name =
                format!("{}.{}", file_name, file.extension) == file.file_name
        }
    }

    handle_validation_result(
        has_parent_with_same_name,
        "rule-parent-has-same-name".to_owned(),
        format!(
            "{} is placed in a folder with the same name",
            file.file_name.bold()
        ),
        format!(
            "Expected {} to be placed in a folder with the same name, but is {}",
            file.file_name.bold(),
            file.relative_path.bold()
        ),
    );
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
    // Check if this file should be skipped for scene node validation
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

            let mut node_name = value.replace("\"", "");
            for (uppercase, pascal_case) in UPPERCASE_TO_PASCAL_CASE.iter() {
                node_name = node_name.replace(uppercase, pascal_case);
            }
            if is_root_node && should_check_root_node_is_file_name_pascal {
                is_root_node = false;
                let file_name_as_pascal_case = file
                    .file_name
                    .replace(&format!(".{}", file.extension).to_string(), "")
                    .to_pascal_case();
                let root_node_has_same_name = file_name_as_pascal_case == node_name;
                handle_validation_result(
                    root_node_has_same_name,
                    "rule-root-node-is-file-name-pascal".to_owned(),
                    format!(
                        "Root node of {} is {}",
                        file.file_name, file_name_as_pascal_case
                    ),
                    format!(
                        "Expected root node of {} to be {}, but was {}",
                        file.file_name, file_name_as_pascal_case, node_name
                    ),
                );
            }
            if should_check_scene_nodes_pascal_case {
                let mut node_name_to_test = node_name.to_owned();
                if node_name_to_test.ends_with("2D") {
                    node_name_to_test = node_name_to_test
                        .strip_suffix("2D")
                        .unwrap_or(&node_name_to_test)
                        .to_owned();
                } else if node_name.ends_with("3D") {
                    node_name_to_test = node_name_to_test
                        .strip_suffix("3D")
                        .unwrap_or(&node_name_to_test)
                        .to_owned();
                }

                for (uppercase, pascal_case) in UPPERCASE_TO_PASCAL_CASE.iter() {
                    node_name_to_test = node_name_to_test.replace(uppercase, pascal_case);
                }

                let mut is_valid = node_name_to_test.is_pascal_case();
                if CONFIG.allow_screaming_snake_case_in_node_names && !is_valid {
                    is_valid = node_name_to_test.is_upper_case()
                        && node_name_to_test.to_lowercase().is_snake_case();
                }

                handle_validation_result(
                    is_valid,
                    "rule-scene-nodes-pascal-case".to_owned(),
                    format!(
                        "Used correct naming-convention for node {} in scene '{}'",
                        node_name.bold(),
                        file.file_name.bold()
                    ),
                    format!(
                        "Expected PascalCase{} naming-convention, but was {} in filename for '{}'",
                        if CONFIG.allow_screaming_snake_case_in_node_names {
                            " or SCREAMING_SNAKE_CASE"
                        } else {
                            ""
                        },
                        node_name.bold(),
                        file.file_name.bold()
                    ),
                );
            }
        }
    }
}

fn validate_name(file: &FileUnderTest) {
    // Check if this file should be skipped for filename validation
    for pattern in CONFIG.ignore_patterns.filename_snake_case.iter() {
        if glob_match(pattern, &file.relative_path) {
            return;
        }
    }

    let is_valid =
        file.file_name.is_snake_case() && file.file_name.chars().all(|c| !c.is_uppercase());

    handle_validation_result(
        is_valid,
        "rule-filename-snake-case".to_owned(),
        format!(
            "{} uses correct lowercase snake_case naming convention",
            file.file_name.bold()
        ),
        format!(
            "Expected lowercase snake_case for {}, but got {}",
            file.file_name.bold(),
            file.file_name.bold()
        ),
    );
}

fn validate_file_root(file: &FileUnderTest) {
    // Check if this file should be skipped for location validation
    for pattern in CONFIG.ignore_patterns.allowed_file_location.iter() {
        if glob_match(pattern, &file.relative_path) {
            return;
        }
    }
    let mut in_correct_root = false;
    let mut can_skip = true;
    // Find matching pattern for this file type
    let mut matched_locations: Vec<String> = vec![];
    for (pattern, locations) in CONFIG.allowed_file_locations.iter() {
        if glob_match(pattern, &file.relative_path) {
            can_skip = false;
            for location in locations {
                matched_locations.push(location.to_owned());
                if glob_match(location, &file.relative_path) {
                    in_correct_root = true;
                }
            }
        }
    }
    if can_skip {
        return;
    }
    let folders_list = matched_locations.join(" or ");

    handle_validation_result(
        in_correct_root,
        "rule-allowed-file-location".to_owned(),
        format!("Found {} in correct location", file.file_name.bold()),
        format!(
            "Expected {} to be in {} but found it in {}",
            file.file_name.bold(),
            folders_list.bold(),
            file.relative_path.bold()
        ),
    );
}

fn handle_validation_result(
    is_success: bool,
    rule_name: String,
    success_message: String,
    error_message: String,
) {
    if is_success {
        if CONFIG.should_print_success {
            println!(
                "\t{} ({}): {}",
                "Test Succesful".green(),
                rule_name.bright_black(),
                success_message
            )
        }
    } else {
        println!(
            "\t{} ({}): {}",
            "Test Failed".red(),
            rule_name.bright_black(),
            error_message
        );
        unsafe {
            VALIDATION_FAILS += 1;
        }
    }

    unsafe { VALIDATION_COUNT += 1 }
}

fn visit_dirs(path_string: &str, dir: &Path, cb: &dyn Fn(&str, &DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        let normalized_path = normalize_path(
            dir.strip_prefix(path_string)
                .unwrap_or(dir)
                .to_str()
                .unwrap_or(""),
        );
        for pattern in IGNORE_PATTERNS.iter() {
            if glob_match(pattern, &normalized_path) {
                return Ok(());
            }
        }

        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(path_string, &path, cb)?;
            } else {
                cb(path_string, &entry);
            }
        }
    }
    Ok(())
}
