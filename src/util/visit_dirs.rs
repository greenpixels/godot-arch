use std::{
    fs::{DirEntry, read_dir},
    io,
    path::Path,
};

use crate::{models::test_results::TestResults, normalize_path};
use glob_match::glob_match;

pub fn visit_dirs(
    path_string: &str,
    ignore_patterns: &Vec<String>,
    dir: &Path,
    test_results: &mut TestResults,
    callback: &dyn Fn(&str, &DirEntry, &mut TestResults),
) -> io::Result<()> {
    if dir.is_dir() {
        let normalized_path = normalize_path(
            dir.strip_prefix(path_string)
                .unwrap_or(dir)
                .to_str()
                .unwrap_or(""),
        );
        for pattern in ignore_patterns.iter() {
            if glob_match(pattern, &normalized_path) {
                return Ok(());
            }
        }

        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(path_string, ignore_patterns, &path, test_results, callback)?;
            } else {
                println!("Running callback");
                callback(path_string, &entry, test_results);
            }
        }
    }
    Ok(())
}
