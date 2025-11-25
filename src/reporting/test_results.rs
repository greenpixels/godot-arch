use serde::Serialize;

use crate::reporting::{report_entry::ReportEntry, warning::Warning};

#[derive(Serialize)]
pub struct TestResults {
    pub files_tested: i32,
    pub files_failed: i32,
    pub warnings: Vec<Warning>,
    pub failed_reports: Vec<ReportEntry>,
    pub successful_reports: Vec<ReportEntry>,
}
