use crate::{
    configuration::config::Config, reporting::check_results::CheckResults,
    rules::handle_validation_result::handle_validation_result,
    util::should_ignore_rule_for_file::should_ignore_rule_for_file,
    validation::file_under_check::FileUnderCheck,
};
use colored::Colorize;
use convert_case::{Case, Casing};

pub fn execute_rule_root_node_is_file_name_pascal(
    node_name: &str,
    file: &FileUnderCheck,
    config: &Config,
    check_results: &mut CheckResults,
) {
    if should_ignore_rule_for_file(
        file,
        Some(
            config
                .include_patterns
                .root_node_is_file_name_pascal
                .to_owned(),
        ),
        Some(
            config
                .ignore_patterns
                .root_node_is_file_name_pascal
                .to_owned(),
        ),
        config,
    ) {
        return;
    }
    // TODO https://github.com/greenpixels/godot-arch/issues/6: Currently we aren't using node_name_pascal_case_exceptions here
    let file_name_as_pascal_case = file
        .file_name
        .replace(&format!(".{}", file.extension).to_string(), "")
        .to_case(Case::Pascal);
    let root_node_has_same_name = file_name_as_pascal_case == node_name;
    handle_validation_result(
        root_node_has_same_name,
        "rule-root-node-is-file-name-pascal".to_owned(),
        format!(
            "Root node of {} is {}",
            file.file_name.bold(),
            file_name_as_pascal_case.bold()
        ),
        format!(
            "Expected root node of {} to be {}, but was {}",
            file.file_name.bold(),
            file_name_as_pascal_case.bold(),
            node_name.bold()
        ),
        check_results,
        file,
    );
}
