use std::process::exit;

use godot_arch::run_godot_arch;

mod interface;
use interface::cli::Args;
enum ExitCode {
    /// Succesfully executed and all checks have passed
    Success = 0,
    /// Succesfully executed with failed checks
    Failure = 1,
    /// Ran into an error while executing
    Error = 2,
}

fn main() {
    let Args {
        config_path,
        project_path,
        report_location,
    } = Args::parse_args();

    match run_godot_arch(&config_path, &project_path, report_location) {
        Ok(check_results) => {
            if check_results.files_failed > 0 {
                exit(ExitCode::Failure as i32)
            } else {
                exit(ExitCode::Success as i32)
            }
        }
        Err(_err) => exit(ExitCode::Error as i32),
    }
}
