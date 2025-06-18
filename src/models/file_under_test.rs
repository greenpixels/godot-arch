use std::path::PathBuf;

pub struct FileUnderTest {
    pub absolute_path: String,
    pub relative_path: String,
    pub file_name: String,
    pub path: PathBuf,
    pub extension: String,
}
