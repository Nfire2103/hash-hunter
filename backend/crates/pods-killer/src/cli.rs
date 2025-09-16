use clap::{Args, Parser, ValueHint};
use humantime::parse_duration;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Config {
    #[clap(flatten, next_help_heading = "Database options")]
    pub database: DatabaseArgs,

    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub kubeconfig: String,

    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub kubenamespace: Option<String>,

    #[arg(long, env, value_parser = parse_duration)]
    pub max_inactive_time: std::time::Duration,
}

#[derive(Args, Debug)]
pub(crate) struct DatabaseArgs {
    #[arg(long, env)]
    pub database_url: String,

    #[arg(long, env)]
    pub database_max_connections: u32,
}
