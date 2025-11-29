use crate::Colorize;
use crate::reporting::test_results::TestResults;

pub fn print_warnings(test_results: &TestResults) {
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
}
