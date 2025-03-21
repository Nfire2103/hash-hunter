#[derive(clap::Parser)]
#[command(version)]
pub struct AppConfig {
    #[clap(long, env)]
    pub address: std::net::SocketAddr,

    #[clap(long, env)]
    pub jwt_secret: String,

    #[clap(flatten, next_help_heading = "Database options")]
    pub database: DatabaseArgs,

    #[clap(flatten, next_help_heading = "Node options")]
    pub node: crate::routes::node::NodeArgs,
}

#[derive(clap::Args)]
pub struct DatabaseArgs {
    #[arg(long, env)]
    pub database_url: String,

    #[arg(long, env)]
    pub database_max_connections: u32,
}
