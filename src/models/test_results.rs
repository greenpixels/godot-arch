use crate::models::warning::Warning;

pub struct TestResults {
    pub files_tested: i32,
    pub files_failed: i32,
    pub warnings: Vec<Warning>,
}
