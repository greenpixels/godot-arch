use colored::Colorize;
use std::fs::exists;
use std::path::Path;
use std::{io, thread};

pub mod configuration;
mod reporting;
mod rules;
#[cfg(test)]
mod tests;
mod util;
mod validation;
use crate::util::ansi::enable_ansi_support_on_windows;
use crate::{
    configuration::config::load_config,
    reporting::{
        check_results::CheckResults, print_summary::print_summary, print_warnings::print_warnings,
        report_writer::write_report,
    },
    util::visit_dirs::visit_dirs,
    validation::process_file::process_file,
};

pub fn run_godot_arch(
    config_path: &str,
    project_path: &str,
    report_location: Option<String>,
) -> Result<CheckResults, Box<dyn std::error::Error>> {
    enable_ansi_support_on_windows();
    let config = load_config(config_path)?;
    let start_time = std::time::Instant::now();

    let mut check_results = CheckResults::default();

    let path = Path::new(project_path);

    if exists(path).is_err() {
        return Err(format!("Path does not exist: {}", project_path).into());
    }

    println!("Indexing in {}", project_path);
    if let Some(files) = visit_dirs(&config, path) {
        let mut handles = vec![];
        for file in files {
            let value_clone = config.clone();
            handles.push(thread::spawn(move || process_file(file, value_clone)));
        }
        for handle in handles {
            if let Ok(result) = handle.join() {
                if result.is_none() {
                    continue;
                }
                check_results.merge(result.unwrap_or(CheckResults::default()));
            }
        }
    }

    print_warnings(&check_results);

    let elapsed_time = start_time.elapsed();
    print_summary(&check_results, elapsed_time);

    if let Some(location) = report_location
        && let Err(e) = write_report(&location, &check_results)
    {
        eprintln!("Error writing report: {}", e);
    }

    if config.wait_for_input_before_close {
        println!("\nPress any button to exit ...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
    Ok(check_results)
}
