use std::{fs::DirEntry, path::PathBuf};

#[derive(Debug, Clone)]
pub struct FileUnderTest {
    pub absolute_path: String,
    pub relative_path: String,
    pub file_name: String,
    pub path: PathBuf,
    pub extension: String,
}

impl FileUnderTest {
    pub fn from_dir_entry(entry: &DirEntry, project_path: &str) -> Self {
        use crate::util::normalize_path::normalize_path;

        let path = entry.path();
        let full_path = path.to_str().unwrap_or("");
        let file_name = entry.file_name().to_string_lossy().to_string();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        let absolute_path = path
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_owned();

        let relative_path =
            normalize_path(full_path.strip_prefix(project_path).unwrap_or(&file_name));

        FileUnderTest {
            absolute_path,
            relative_path,
            file_name,
            path: path.to_owned(),
            extension,
        }
    }
}
