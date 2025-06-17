use k8s_openapi::api::{batch::v1::Job, core::v1::Node};
use kube::{config::InferConfigError, Api, Client, Config};
use tracing::debug;

use crate::crds::{FlatboatBot, FlatboatWorkload};

pub struct OperatorApis {
    pub nodes: Api<Node>,
    pub jobs: Api<Job>,
    pub workload: Api<FlatboatWorkload>,
    pub bots: Api<FlatboatBot>,
    pub client: kube::Client,
}

#[derive(thiserror::Error, Debug)]
pub enum ApisInitError {
    #[error("Failed to infer Kubernetes configuration: {0}")]
    ConfigError(#[from] InferConfigError),

    #[error("Unable to connect Kubernetes client: {0}")]
    KubeError(#[from] kube::Error),
}

pub async fn initialize_apis() -> Result<OperatorApis, ApisInitError> {
  // 1. Create Kubernetes client
  debug!("Creating Kubernetes client...");
  let config = Config::infer().await?;
  let client = Client::try_from(config)?;
  debug!("Kubernetes client created successfully");

  // 2. Initialize Kubernetes APIs
  let nodes_apis: Api<Node> = Api::all(client.clone());
  let jobs_apis: Api<Job> = Api::all(client.clone());
  let workload_apis: Api<FlatboatWorkload> = Api::all(client.clone());
  let bots_apis: Api<FlatboatBot> = Api::all(client.clone());

  return Ok(
    OperatorApis {
      nodes: nodes_apis,
      jobs: jobs_apis,
      workload: workload_apis,
      bots: bots_apis,
      client,
    }
  );
}