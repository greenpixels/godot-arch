use colored::Colorize;
use std::fs::exists;
use std::path::Path;
use std::{io, vec};

mod interface;
mod reporting;
mod rules;
#[cfg(test)]
mod tests;
mod util;
mod validation;

use crate::interface::cli::Args;
use crate::interface::config::Config;
use crate::reporting::report_writer::write_report;
use crate::reporting::test_results::TestResults;
use crate::util::ansi::enable_ansi_support;
use crate::util::visit_dirs::visit_dirs;
use crate::validation::file_processor::handle_file;

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

fn run_validation(
    project_path: &str,
    config: &Config,
    test_results: &mut TestResults,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(project_path);

    if exists(path).is_err() {
        return Err(format!("Path does not exist: {}", project_path).into());
    }

    println!("Indexing in {}", project_path);
    visit_dirs(project_path, config, path, test_results, &handle_file)?;
    Ok(())
}

fn print_warnings(test_results: &TestResults) {
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

fn print_summary(test_results: &TestResults, elapsed_time: std::time::Duration) {
    println!(
        "\n>\t{} tests of {} total have failed",
        test_results.files_failed, test_results.files_tested
    );
    println!("Total execution time: {:.2?}", elapsed_time);
}

fn wait_for_user_input() {
    println!("\nPress any button to exit ...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();

    let Args {
        config_path,
        project_path,
        report_location,
    } = Args::parse_args();

    let config = load_config(&config_path)?;
    let start_time = std::time::Instant::now();

    let mut test_results = TestResults {
        files_tested: 0,
        files_failed: 0,
        warnings: vec![],
        failed_reports: vec![],
        successful_reports: vec![],
    };

    run_validation(&project_path, &config, &mut test_results)?;

    print_warnings(&test_results);

    let elapsed_time = start_time.elapsed();
    print_summary(&test_results, elapsed_time);

    if config.wait_for_input_before_close {
        wait_for_user_input();
    }

    if let Some(ref location) = report_location
        && let Err(e) = write_report(location, &test_results)
    {
        eprintln!("Error writing report: {}", e);
    }

    if test_results.files_failed != 0 {
        return Err("Some tests were not successful".into());
    }

    Ok(())
}
