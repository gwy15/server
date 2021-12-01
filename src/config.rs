use std::net::SocketAddr;

fn default_bind() -> Vec<SocketAddr> {
    vec!["127.0.0.1:80".parse().unwrap(), "[::]:80".parse().unwrap()]
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The number of worker threads, default to the number of logical CPU
    pub workers: Option<usize>,

    /// The address to listen. Default as ["127.0.0.1:80", "[::]:80"]
    #[serde(default = "default_bind")]
    pub bind: Vec<SocketAddr>,

    pub db_uri: String,

    pub db_max_connections: Option<u32>,
}
