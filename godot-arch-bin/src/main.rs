use colored::Colorize;
use godot_arch::{run_godot_arch, interface::cli::Args};

mod util;
use util::ansi::enable_ansi_support;

fn main() {
    enable_ansi_support();

    let Args {
        config_path,
        project_path,
        report_location,
    } = Args::parse_args();

    run_godot_arch(&config_path, &project_path, report_location).unwrap();
}
