use crate::reporting::check_results::CheckResults;

pub fn print_summary(check_results: &CheckResults, elapsed_time: std::time::Duration) {
    println!(
        "\n>\t{} checks of {} total have failed",
        check_results.files_failed, check_results.files_checked
    );
    println!("Total execution time: {:.2?}", elapsed_time);
}
