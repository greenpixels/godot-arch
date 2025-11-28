use godot_arch::run_godot_arch;

mod interface;
use interface::cli::Args;

fn main() {
    let Args {
        config_path,
        project_path,
        report_location,
    } = Args::parse_args();

    run_godot_arch(&config_path, &project_path, report_location).unwrap();
}
