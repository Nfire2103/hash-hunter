use anyhow::Result;
use k8s_openapi::api::{
    apps::v1::{Deployment, ReplicaSet},
    core::v1::Pod,
};
use kube::{
    Api, Client, Config,
    api::DeleteParams,
    config::{KubeConfigOptions, Kubeconfig},
};
use tokio::task;
use tracing::{error, info};

use crate::cli;

pub(crate) struct KubeHandler {
    client: Client,
    pods: Api<Pod>,
    replica_sets: Api<ReplicaSet>,
    deployments: Api<Deployment>,
}

impl TryFrom<cli::Config> for KubeHandler {
    type Error = anyhow::Error;

    fn try_from(args: cli::Config) -> anyhow::Result<Self> {
        let kubeconfig = Kubeconfig::read_from(args.kubeconfig)?;
        let config = task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(Config::from_custom_kubeconfig(
                kubeconfig,
                &KubeConfigOptions::default(),
            ))
        })?;

        let client = Client::try_from(config)?;
        let namespace = args.kubenamespace.unwrap_or("default".to_string());

        Ok(Self {
            pods: Api::namespaced(client.clone(), &namespace),
            replica_sets: Api::namespaced(client.clone(), &namespace),
            deployments: Api::namespaced(client.clone(), &namespace),
            client,
        })
    }
}

impl KubeHandler {
    pub(crate) async fn get_pod(&self, name: &str) -> Result<Pod> {
        Ok(self.pods.get(name).await?)
    }

    pub(crate) async fn stop_pod(&self, name: &str) -> Result<()> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), "default");

        let pod = pods.get(name).await.map_err(|err| {
            error!("Failed to get pod '{}': {:?}", name, err);
            err
        })?;

        let rs_name = pod
            .metadata
            .owner_references
            .as_ref()
            .and_then(|owners| owners.iter().find(|o| o.kind == "ReplicaSet"))
            .ok_or_else(|| {
                let msg = format!("ReplicaSet not found for pod '{name}'");
                error!("ReplicaSet not found for pod '{}'", msg);
                anyhow::anyhow!(msg)
            })?
            .name
            .clone();

        let rs = self.replica_sets.get(&rs_name).await?;

        let deployment_name = rs
            .metadata
            .owner_references
            .as_ref()
            .and_then(|owners| owners.iter().find(|o| o.kind == "Deployment"))
            .ok_or_else(|| {
                let msg = format!("Deployment not found for ReplicaSet '{rs_name}'");
                error!("{}", msg);
                anyhow::anyhow!(msg)
            })?
            .name
            .clone();

        self.deployments
            .delete(&deployment_name, &DeleteParams::default())
            .await?;

        pods.delete(name, &DeleteParams::default()).await?;

        info!("Pod '{}' successfully stopped", name);

        Ok(())
    }
}
