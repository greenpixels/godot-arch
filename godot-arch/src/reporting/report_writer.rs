use std::path::Path;

use crate::reporting::check_results::CheckResults;

pub fn write_report(report_location: &str, check_results: &CheckResults) -> Result<(), String> {
    let report_path = Path::new(report_location).join("godot-arch-report.json");
    println!("Writing report to {}", report_path.display());

    let report_json = serde_json::to_string_pretty(check_results)
        .map_err(|e| format!("Failed to serialize report: {}", e))?;

    if let Some(parent) = report_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create report directory: {}", e))?;
    }

    std::fs::write(&report_path, report_json)
        .map_err(|e| format!("Failed to write report file: {}", e))?;

    Ok(())
}
