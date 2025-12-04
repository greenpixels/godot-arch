use colored::Colorize;
use std::fs::exists;
use std::path::Path;
use std::{io, vec};

pub mod configuration;
mod reporting;
mod rules;
#[cfg(test)]
mod tests;
mod util;
mod validation;

use crate::util::ansi::enable_ansi_support;
use crate::{
    configuration::config::load_config,
    reporting::{
        print_summary::print_summary, print_warnings::print_warnings, report_writer::write_report,
        test_results::TestResults,
    },
    util::visit_dirs::visit_dirs,
    validation::process_file::process_file,
};

pub fn run_godot_arch(
    config_path: &str,
    project_path: &str,
    report_location: Option<String>,
) -> Result<TestResults, Box<dyn std::error::Error>> {
    enable_ansi_support();
    let config = load_config(config_path)?;
    let start_time = std::time::Instant::now();

    let mut test_results = TestResults {
        files_tested: 0,
        files_failed: 0,
        warnings: vec![],
        failed_reports: vec![],
        successful_reports: vec![],
    };

    let path = Path::new(project_path);

    if exists(path).is_err() {
        return Err(format!("Path does not exist: {}", project_path).into());
    }

    println!("Indexing in {}", project_path);
    visit_dirs(
        project_path,
        &config,
        path,
        &mut test_results,
        &process_file,
    )?;
    print_warnings(&test_results);

    let elapsed_time = start_time.elapsed();
    print_summary(&test_results, elapsed_time);

    if let Some(location) = report_location
        && let Err(e) = write_report(&location, &test_results)
    {
        eprintln!("Error writing report: {}", e);
    }

    if config.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    Ok(test_results)
}
