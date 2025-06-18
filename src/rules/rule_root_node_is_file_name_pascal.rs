use crate::{
    models::{config::Config, file_under_test::FileUnderTest, test_results::TestResults},
    rules::handle_validation_result::handle_validation_result,
};
use convert_case::{Case, Casing};

pub fn rule_root_node_is_file_name_pascal(
    node_name: &str,
    file: &FileUnderTest,
    config: &Config,
    test_results: &mut TestResults,
) {
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
            file.file_name, file_name_as_pascal_case
        ),
        format!(
            "Expected root node of {} to be {}, but was {}",
            file.file_name, file_name_as_pascal_case, node_name
        ),
        config,
        test_results,
    );
}
