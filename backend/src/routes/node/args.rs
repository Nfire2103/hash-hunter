use clap::ValueHint;

#[derive(clap::Args)]
pub struct NodeArgs {
    #[arg(long, env, value_hint = ValueHint::FilePath)]
    pub kubeconfig: String,

    #[clap(long, env, value_hint = ValueHint::FilePath)]
    pub templates: String,

    #[clap(long, env, value_hint = ValueHint::FilePath)]
    pub deployment_file: String,

    #[clap(long, env, value_hint = ValueHint::FilePath)]
    pub service_file: String,
}
