use crate::reporting::test_results::TestResults;

pub fn print_summary(test_results: &TestResults, elapsed_time: std::time::Duration) {
    println!(
        "\n>\t{} tests of {} total have failed",
        test_results.files_failed, test_results.files_tested
    );
    println!("Total execution time: {:.2?}", elapsed_time);
}
