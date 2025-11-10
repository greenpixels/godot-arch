use predicates::*;

use crate::{rules::handle_validation_result, tests::mocks::get_test_results_mock};

#[test]
fn test_should_correctly_handle_failed_validation() {
    let mut test_results = get_test_results_mock();
    let validation_result = handle_validation_result::handle_validation_result(
        false,
        String::new(),
        String::new(),
        String::new(),
        true,
        &mut test_results,
    );
    let predicate = prelude::predicate::str::contains("Failed").count(1);
    assert_eq!(true, validation_result.is_some());
    assert_eq!(true, predicate.eval(&validation_result.unwrap()));
    assert_eq!(test_results.files_failed, 1);
    assert_eq!(test_results.files_tested, 1);
}

#[test]
fn test_should_correctly_handle_successful_validation() {
    let mut test_results = get_test_results_mock();
    let validation_result = handle_validation_result::handle_validation_result(
        true,
        String::new(),
        String::new(),
        String::new(),
        true,
        &mut test_results,
    );
    let predicate = prelude::predicate::str::contains("Succesful").count(1);
    assert_eq!(true, validation_result.is_some());
    assert_eq!(true, predicate.eval(&validation_result.unwrap()));
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}

#[test]
fn test_should_not_output_on_success_when_flag_is_false() {
    let mut test_results = get_test_results_mock();
    let validation_result = handle_validation_result::handle_validation_result(
        true,
        String::new(),
        String::new(),
        String::new(),
        false,
        &mut test_results,
    );
    assert_eq!(true, !validation_result.is_some());
}

#[test]
fn test_should_output_on_failed_when_flag_is_false() {
    let mut test_results = get_test_results_mock();
    let validation_result = handle_validation_result::handle_validation_result(
        false,
        String::new(),
        String::new(),
        String::new(),
        false,
        &mut test_results,
    );
    let predicate = prelude::predicate::str::contains("Failed").count(1);
    assert_eq!(true, validation_result.is_some());
    assert_eq!(true, predicate.eval(&validation_result.unwrap()));
}
