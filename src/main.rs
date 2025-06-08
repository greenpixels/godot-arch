use colored::Colorize;
use inflections::Inflect;
use std::env;
use std::fs::{exists, read_dir, DirEntry};
use std::io;
use std::path::{Path, PathBuf};


static mut VALIDATION_COUNT: i32 = 0;
static mut VALIDATION_FAILS: i32 = 0;
const SHOULD_PRINT_SUCCESS : bool = false;
struct FileUnderTest {
    absolute_path: String,
    relative_path: String,
    file_name: String,
    path: PathBuf,
    expected_root_folder: String,
}


#[cfg(windows)]
fn enable_ansi_support() {
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
    println!("\n>\t{} tests of {} total have failed", unsafe {VALIDATION_FAILS}, unsafe{VALIDATION_COUNT});
    
    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:.2?}", elapsed_time);
    
    println!("\nPress any button to exit ...");
    io::stdin().read_line(&mut String::new()).unwrap();
    
    if unsafe{VALIDATION_FAILS != 0 } {
         return Err("Some tests were not succesful".into());
    }
    return Ok(())
}

fn handle_file(input_path_string: &str, entry: &DirEntry) {
    let path = entry.path();
    let full_path = path.to_str().unwrap_or("");
    let file_name = path
        .file_name()
        .and_then(|comp| comp.to_str())
        .unwrap_or("");

    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let expected_root_folder = match extension {
        "tscn" => "scenes",
        ext if is_image_extension(ext) => "assets\\images",
        ext if is_audio_extension(ext) => "assets\\audio",
        "tres" => "resources",
        _ => "",
    };
    let file_under_test = FileUnderTest {
        file_name: file_name.to_owned(),
        path: path.to_owned(),
        absolute_path: path.canonicalize().unwrap().to_str().unwrap_or("").to_owned(),
        relative_path: full_path
            .strip_prefix(input_path_string)
            .unwrap_or(file_name)
            .to_owned(),
        expected_root_folder: expected_root_folder.to_owned(),
    };
    validate_file(file_under_test, extension)
}

fn is_image_extension(ext: &str) -> bool {
    matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "gif" | "webp")
}


fn is_audio_extension(ext: &str) -> bool {
    matches!(ext.to_lowercase().as_str(), "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a")
}


fn validate_file(file: FileUnderTest, extension: &str) {
    if should_skip_file(&file) {
        return
    }
    validate_file_parent(&file);
    validate_name(&file);
    match extension  {
        "tscn" => {
            validate_parent_has_same_name(&file);
            validate_scene_nodes(file);
        },
        "tres" => {
            validate_parent_has_same_name(&file);
        },
        "gd" => {
            validate_parent_has_same_name(&file);
        }
        _ => return
    }
    if extension == "tscn" {
        
        
    }
}

fn validate_parent_has_same_name(file: &FileUnderTest) {
    let parent = file.path.parent();
    let has_parent_with_same_name : bool = match parent {
        Some(parent) => format!("{}.tscn", parent.file_name().unwrap().to_str().unwrap_or("")) == file.file_name,
        None => false
    };
    print_validation_result(
        has_parent_with_same_name,
        format!("{} is placed in a folder with the same name", file.file_name.yellow()),
        format!("Expected {} to be placed in a folder with the same name, but is {}", file.file_name.yellow(), file.relative_path.yellow())
    );
}

fn should_skip_file(file: &FileUnderTest) -> bool {
    return file.relative_path.starts_with("\\export") || file.relative_path.starts_with("\\.") || file.relative_path.starts_with("\\addons") 
}

fn validate_scene_nodes(file: FileUnderTest) {
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
                node_name_to_test = node_name_to_test.strip_suffix("2D").unwrap_or(&node_name_to_test).to_owned();
            } else if node_name.ends_with("3D") {
                node_name_to_test = node_name_to_test.strip_suffix("3D").unwrap_or(&node_name_to_test).to_owned();
            }
            print_validation_result(
                node_name_to_test.is_pascal_case(),
                format!(
                    "Used PascalCase naming-convention for node {} in scene '{}'",
                    node_name.yellow(), file.file_name.yellow()
                ),
                format!(
                    "Expected PascalCase naming-convention, but was {} in filename for '{}'",
                    node_name.yellow(), file.file_name.yellow()
                ),
            );
        }
    }
}

fn validate_name(file: &FileUnderTest) {
    print_validation_result(
        file.file_name.is_snake_case(),
        format!(
            "Used snake_case naming-convention in filename for '{}'",
            file.file_name.yellow()
        ),
        format!(
            "Expected snake_case naming-convention in filename, but was '{}'",
            file.file_name.yellow()
        ),
    );
}

fn validate_file_parent(file: &FileUnderTest) {
    let in_correct_root = file
        .relative_path
        .strip_prefix("\\")
        .unwrap_or(&file.relative_path)
        .starts_with(&file.expected_root_folder);

    print_validation_result(
        in_correct_root,
        format!(
            "Found {} in \\{}",
            file.file_name.yellow(), file.expected_root_folder.yellow()
        ),
        format!(
            "Expected {} to be in \\{} directory, but was instead in {}",
            file.file_name.yellow(), file.expected_root_folder.yellow(), file.absolute_path.yellow()
        ),
    );
}

fn print_validation_result(is_success: bool, success_message: String, error_message: String) {
    if is_success {
        if SHOULD_PRINT_SUCCESS {
             println!("\n{}: {}", "Test Succesful".green(), success_message)
        }
    } else {
        println!("\n{}: {}", "Test Failed".red(), error_message);
        unsafe { VALIDATION_FAILS += 1; }
    }

    unsafe {VALIDATION_COUNT += 1}
}

fn visit_dirs(path_string: &str, dir: &Path, cb: &dyn Fn(&str, &DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
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
