use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ReportEntry {
    pub absolute_file_path: String,
    pub rule_name: String,
    pub message: String,
}
