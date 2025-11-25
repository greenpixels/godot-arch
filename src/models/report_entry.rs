use serde::Serialize;

#[derive(Serialize)]
pub struct ReportEntry {
    pub absolute_file_path: String,
    pub rule_name: String,
    pub message: String,
}
