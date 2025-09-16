mod cli;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use clap::Parser;
use cli::Config;
use futures::TryStreamExt;
use sqlx::{PgPool, postgres::PgPoolOptions, types::uuid};
use tracing::{debug, info, warn};
mod kube_handler;
use kube_handler::KubeHandler;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Node {
    pub id: Uuid,
    pub pod_name: Option<String>,
    pub last_activity: NaiveDateTime,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let _ = dotenvy::from_path(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env"));

    let args = Config::parse();
    let max_inactive_time = args.max_inactive_time;

    let pool = PgPoolOptions::new()
        .max_connections(args.database.database_max_connections)
        .connect_lazy(&args.database.database_url)?;

    let kube = KubeHandler::try_from(args)?;

    process_nodes(&pool, &kube, max_inactive_time).await?;

    Ok(())
}

async fn process_nodes(pool: &PgPool, kube: &KubeHandler, max_inactive_time: Duration) -> Result<()> {
    let mut stream =
        sqlx::query_as::<_, Node>("SELECT id, pod_name, pod_uid, last_activity FROM node").fetch(pool);

    while let Some(node) = stream.try_next().await? {
        if node.pod_name.is_some() {
            process_active_node(&node, pool, kube, max_inactive_time).await?;
        } else {
            process_missing_node(&node, pool).await?;
        }
    }

    Ok(())
}

async fn process_active_node(
    node: &Node,
    pool: &PgPool,
    kube: &KubeHandler,
    max_inactive_time: Duration,
) -> Result<()> {
    if let Some(name) = &node.pod_name {
        let id = node.id;
        let last_activity = node.last_activity;

        let now = Utc::now().naive_utc();
        let inactive_duration_chrono = now - last_activity;

        let inactive_duration = match inactive_duration_chrono.to_std() {
            Ok(dur) => dur,
            Err(_) => {
                warn!(pod_name = %name, last_activity = %last_activity, "last_activity is in the future, skipping");
                return Ok(());
            },
        };

        if inactive_duration <= max_inactive_time {
            debug!(pod_name = %name, last_activity = %last_activity, "Pod still active, skipping");
            return Ok(());
        }

        debug!(pod_name = %name, last_activity = %last_activity, "Inactive pod found");

        match kube.get_pod(name).await {
            Ok(_) => {
                info!(pod_name = %name, "Stopping inactive pod");
                if let Err(e) = kube.stop_pod(name).await {
                    warn!(error = %e, pod_name = %name, "Failed to stop pod");
                }
            },
            Err(e) => {
                warn!(error = %e, pod_name = %name, "Pod lookup failed, deleting node entry");
                delete_node(pool, id).await?;
            },
        }
    }
    Ok(())
}

async fn process_missing_node(node: &Node, pool: &PgPool) -> Result<()> {
    let Node { id, .. } = node;
    warn!(%id, "Pod missing, deleting node entry");
    delete_node(pool, *id).await?;
    Ok(())
}

async fn delete_node(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM node WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    info!(%id, "Node deleted from database");
    Ok(())
}
