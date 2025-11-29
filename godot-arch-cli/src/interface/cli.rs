use clap::{Parser, arg};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to godot project root
    #[arg(short = 'p', long = "project", default_value_t = String::from("./"))]
    pub project_path: String,

    /// Path to configuration file
    #[arg(
        short = 'c',
        long = "config",
        default_value_t = String::from("./godot-arch.config.yaml")
    )]
    pub config_path: String,

    /// Location in which to save a report in
    #[arg(long = "report")]
    pub report_location: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
