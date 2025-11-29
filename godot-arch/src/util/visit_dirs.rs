use std::{
    fs::{DirEntry, read_dir},
    io,
    path::Path,
};

use crate::{
    configuration::config::Config, reporting::test_results::TestResults,
    util::normalize_path::normalize_path,
};
use glob_match::glob_match;

pub fn visit_dirs(
    path_string: &str,
    config: &Config,
    dir: &Path,
    test_results: &mut TestResults,
    callback: &dyn Fn(&str, &DirEntry, &mut TestResults, &Config),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
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
                visit_dirs(path_string, config, &path, test_results, callback)?;
            } else {
                callback(path_string, &entry, test_results, config);
            }
        }
    }
    Ok(())
}
