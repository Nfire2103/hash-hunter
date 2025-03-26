use std::sync::Arc;

use anyhow::Result;
use kube::config::{KubeConfigOptions, Kubeconfig};
use tera::Tera;

use super::args::NodeArgs;

#[derive(Clone)]
pub struct NodeState {
    pub http_client: reqwest::Client,
    pub kube_client: kube::Client,
    pub tera: Arc<Tera>,
    pub deployment_file: String,
    pub service_file: String,
}

impl NodeState {
    pub async fn try_from_args(args: NodeArgs) -> Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            kube_client: load_kubeconfig(&args.kubeconfig).await?,
            tera: Arc::new(Tera::new(&args.templates)?),
            deployment_file: args.deployment_file,
            service_file: args.service_file,
        })
    }
}

async fn load_kubeconfig(path: &str) -> Result<kube::Client> {
    let kubeconfig = Kubeconfig::read_from(path)?;
    let config = kube::Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

    let client = kube::Client::try_from(config)?;
    Ok(client)
}
