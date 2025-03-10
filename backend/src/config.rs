use clap::{Parser, ValueHint};

#[derive(Parser)]
#[command(version)]
pub struct Config {
    #[clap(long, env)]
    pub address: std::net::SocketAddr,

    #[arg(long, env)]
    pub db_uri: String,

    #[arg(long, env)]
    pub db_max_connections: u32,

    #[clap(long, env, value_hint = ValueHint::Url)]
    pub anvil_uri: String,
}
