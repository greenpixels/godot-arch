use serde::Serialize;

use crate::reporting::{report_entry::ReportEntry, warning::Warning};

#[derive(Serialize, Default, Clone)]
pub struct TestResults {
    pub files_tested: i32,
    pub files_failed: i32,
    pub warnings: Vec<Warning>,
    pub failed_reports: Vec<ReportEntry>,
    pub successful_reports: Vec<ReportEntry>,
}

impl TestResults {
    pub fn merge(&mut self, delta: TestResults) {
        self.files_tested += delta.files_tested;
        self.files_failed += delta.files_failed;
        self.warnings.extend(delta.warnings);
        self.failed_reports.extend(delta.failed_reports);
        self.successful_reports.extend(delta.successful_reports);
    }
}
