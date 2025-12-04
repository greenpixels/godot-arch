use std::{fs::read_dir, path::Path};

use crate::{
    configuration::config::Config, util::normalize_path::normalize_path,
    validation::file_under_test::FileUnderTest,
};
use glob_match::glob_match;

pub fn visit_dirs(config: &Config, dir: &Path) -> Option<Vec<FileUnderTest>> {
    visit_dirs_internal(config, dir, dir)
}

fn visit_dirs_internal(
    config: &Config,
    project_root: &Path,
    dir: &Path,
) -> Option<Vec<FileUnderTest>> {
    if !dir.is_dir() {
        return None;
    }
    let read_dir_iter = match read_dir(dir) {
        Ok(iter) => iter,
        Err(_) => return None,
    };
    let mut files: Vec<FileUnderTest> = vec![];
    let project_path = project_root.to_str().unwrap_or("");
    for entry in read_dir_iter {
        let Some(entry) = entry.ok() else { continue };
        let path = entry.path();

        let normalized_path = normalize_path(
            dir.strip_prefix(path.display().to_string())
                .unwrap_or(dir)
                .to_str()
                .unwrap_or(""),
        );

        if config
            .ignore_patterns
            .overall
            .iter()
            .any(|pattern| glob_match(pattern, &normalized_path))
        {
            continue;
        }

        if path.is_dir() {
            if let Some(dir_files) = visit_dirs_internal(config, project_root, &path) {
                files.extend(dir_files);
            }
        } else {
            files.push(FileUnderTest::from_dir_entry(&entry, project_path));
        }
    }

    Some(files)
}
