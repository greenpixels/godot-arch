use crate::interface::config::Config;
use crate::validation::file_under_test::FileUnderTest;
use glob_match::glob_match;

pub fn should_ignore_rule_for_file(
    file: &FileUnderTest,
    include_patterns: Option<Vec<String>>,
    ignore_patterns: Option<Vec<String>>,
    config: &Config,
) -> bool {
    if config
        .ignore_patterns
        .overall
        .iter()
        .any(|pattern| glob_match(pattern, &file.relative_path))
    {
        return true;
    }

    if let Some(ignore_patterns) = &ignore_patterns
        && ignore_patterns
            .iter()
            .any(|pattern| glob_match(pattern, &file.relative_path))
    {
        return true;
    }

    if let Some(include_patterns) = &include_patterns
        && !include_patterns
            .iter()
            .any(|pattern| glob_match(pattern, &file.relative_path))
    {
        return true;
    }

    false
}
