use godot_properties_parser::parsers::parser_property_file::PropertyFile;

use crate::{
    configuration::config::Config, reporting::check_results::CheckResults,
    rules::rule_allowed_custom_resource_location::execute_rule_allowed_custom_resource_location,
    validation::file_under_check::FileUnderCheck,
};

pub fn validate_resource_file(
    parsed_resource_file: PropertyFile,
    file: &FileUnderCheck,
    check_results: &mut CheckResults,
    config: &Config,
) {
    let header_type = match parsed_resource_file.sections.first() {
        Some(section) => section.header_type.as_str(),
        None => return,
    };

    if !header_type.eq("gd_resource") {
        return;
    }
    let Some(section) = parsed_resource_file.sections.first() else {
        return;
    };
    let Some(script_class_property) = section.properties.iter().find(|p| p.key == "script_class")
    else {
        return;
    };
    execute_rule_allowed_custom_resource_location(
        script_class_property.value.as_str(),
        file,
        config,
        check_results,
    );
}
