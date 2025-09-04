use clap::{Args, ValueHint};

#[derive(Debug, Args)]
pub struct NodeArgs {
    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub kubeconfig: String,

    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub templates: String,

    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub deployment_file: String,

    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub service_file: String,
}
