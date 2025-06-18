use crate::models::file_under_test::FileUnderTest;
use glob_match::glob_match;

pub fn should_ignore_rule_for_file(
    file: &FileUnderTest,
    include_patterns: Option<Vec<String>>,
    ignore_patterns: Option<Vec<String>>,
) -> bool {
    if let Some(include_patterns) = &include_patterns {
        if !include_patterns
            .iter()
            .any(|pattern| glob_match(pattern, &file.relative_path))
        {
            return true;
        }
    }

    if let Some(ignore_patterns) = &ignore_patterns {
        if ignore_patterns
            .iter()
            .any(|pattern| glob_match(pattern, &file.relative_path))
        {
            return true;
        }
    }
    return false;
}
