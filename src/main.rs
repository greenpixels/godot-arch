use colored::Colorize;
use glob::Pattern;
use inflections::Inflect;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::{DirEntry, exists, read_dir};
use std::io;
use std::path::{Path, PathBuf};

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
}

static mut VALIDATION_COUNT: i32 = 0;
static mut VALIDATION_FAILS: i32 = 0;
const SHOULD_PRINT_SUCCESS: bool = true;

lazy_static::lazy_static! {
    static ref CONFIG: Config = {
        let config_content = std::fs::read_to_string("godot-arch.config.yaml")
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
    };

    static ref IGNORE_PATTERNS: Vec<Pattern> = {
        CONFIG.ignore_patterns.overall
            .iter()
            .filter_map(|p| Pattern::new(p).ok())
            .collect()
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

    let args: Vec<String> = env::args().collect();
    let path_string: &str;
    if args.len() > 1 {
        path_string = &args[1];
    } else {
        path_string = ".";
    }
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

    println!("\nPress any button to exit ...");
    io::stdin().read_line(&mut String::new()).unwrap();

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
        if let Ok(pattern) = Pattern::new(pattern) {
            if pattern.matches(&file.relative_path) {
                return;
            }
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
    for pattern in IGNORE_PATTERNS.iter() {
        if pattern.matches(&file.relative_path) {
            return true;
        }
    }
    false
}

fn validate_scene_nodes(file: FileUnderTest) {
    // Check if this file should be skipped for scene node validation
    for pattern in CONFIG.ignore_patterns.scene_nodes_pascal_case.iter() {
        if let Ok(pattern) = Pattern::new(pattern) {
            if pattern.matches(&file.relative_path) {
                return;
            }
        }
    }

    let file_content = match std::fs::read_to_string(&file.absolute_path) {
        Ok(content) => content,
        Err(_) => return,
    };

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

            // Replace any uppercase exceptions with their PascalCase equivalents
            if let Some(pascal_case) = UPPERCASE_TO_PASCAL_CASE.get(&node_name_to_test) {
                node_name_to_test = pascal_case.to_string();
            }

            let is_valid = if CONFIG.allow_screaming_snake_case_in_node_names {
                node_name_to_test.is_pascal_case()
                    || (node_name_to_test.is_snake_case()
                        && node_name_to_test.chars().all(|c| !c.is_lowercase()))
            } else {
                node_name_to_test.is_pascal_case()
            };

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

fn validate_name(file: &FileUnderTest) {
    // Check if this file should be skipped for filename validation
    for pattern in CONFIG.ignore_patterns.filename_snake_case.iter() {
        if let Ok(pattern) = Pattern::new(pattern) {
            if pattern.matches(&file.relative_path) {
                return;
            }
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
        if let Ok(pattern) = Pattern::new(pattern) {
            if pattern.matches(&file.relative_path) {
                return;
            }
        }
    }

    // Find matching pattern for this file type
    let mut matched_locations = Vec::new();
    for (pattern, locations) in CONFIG.allowed_file_locations.iter() {
        if let Ok(glob_pattern) = Pattern::new(pattern) {
            if glob_pattern.matches(&file.relative_path) {
                matched_locations.extend(locations.iter().cloned());
            }
        }
    }

    if matched_locations.is_empty() {
        return;
    }

    let in_correct_root = matched_locations.iter().any(|loc: &String| {
        if let Ok(pattern) = Pattern::new(loc) {
            if pattern.matches(&file.relative_path) {
                return true;
            }
        }
        false
    });

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
        if SHOULD_PRINT_SUCCESS {
            println!(
                "\t{} ({}): {}",
                "Test Succesful".green(),
                rule_name.bright_black(),
                success_message
            )
        }
    } else {
        println!("\t{}: {}", "Test Failed".red(), error_message);
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
            if pattern.matches(&normalized_path) {
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
