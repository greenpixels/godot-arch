use crate::{models::test_results::TestResults, rules::handle_validation_result};

#[test]
fn test_should_increase_files_failed_on_fail() {
    let mut test_results: TestResults = TestResults {
        files_tested: 0,
        files_failed: 0,
    };
    handle_validation_result::handle_validation_result(
        false,
        String::new(),
        String::new(),
        String::new(),
        true,
        &mut test_results,
    );
    assert_eq!(test_results.files_failed, 1);
    assert_eq!(test_results.files_tested, 1);
}

#[test]
fn test_should_not_increase_files_failed_on_success() {
    let mut test_results: TestResults = TestResults {
        files_tested: 0,
        files_failed: 0,
    };
    handle_validation_result::handle_validation_result(
        true,
        String::new(),
        String::new(),
        String::new(),
        true,
        &mut test_results,
    );
    assert_eq!(test_results.files_tested, 1);
    assert_eq!(test_results.files_failed, 0);
}
