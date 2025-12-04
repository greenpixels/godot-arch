use crate::Colorize;
use crate::reporting::check_results::CheckResults;

pub fn print_warnings(check_results: &CheckResults) {
    if !check_results.warnings.is_empty() {
        for warning in &check_results.warnings {
            println!(
                "{} {}\n>>>     in {}",
                "Warning:".yellow(),
                warning.message.yellow(),
                warning.absolute_path.bright_black()
            );
        }
    }
}
