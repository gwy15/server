//! The cli arguments
use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// The path to the config toml file
    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    pub config: String,
}
