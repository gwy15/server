//! The cli arguments
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser)]
pub struct Opts {
    /// The bind address
    #[clap(short = 'b', long = "bind", default_value = "0.0.0.0:8080")]
    pub bind: SocketAddr,

    /// The number of workers
    #[clap(short = 'w', long = "workers")]
    pub workers: Option<usize>,
}
