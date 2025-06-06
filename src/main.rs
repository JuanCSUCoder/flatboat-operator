mod apis;
mod crds;
mod controller;

use std::{error, sync::Arc};

use kube::{api::ListParams, runtime::{watcher::Config, Controller}};
use tracing::{debug, info, warn};
use futures::StreamExt;

use crate::apis::ApisInitError;

#[derive(thiserror::Error, Debug)]
enum OperatorError {
    #[error("Unable to initialize Kubernetes APIs: {0}")]
    KubeError(#[from] ApisInitError),
}

type ProgramResult = Result<(), OperatorError>;

#[tokio::main]
async fn main() -> ProgramResult {
    // 1. Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();
    info!("Starting flatboat operator...");

    // 2. Initialize Kubernetes APIs
    info!("Initializing Kubernetes APIs...");
    let apis = apis::initialize_apis().await?;
    debug!("Kubernetes APIs initialized successfully");

    // 3. Create the CRD controller
    info!("Operator started successfully!");
    let _ = 
        Controller::new(apis.workload.clone(), Config::default())
            .owns(apis.jobs.clone(), Config::default())
            .owns(apis.nodes.clone(), Config::default())
            .run(
                controller::reconcile,
                controller::error_policy,
                Arc::new(apis)
            ).for_each(|res| async move {}).await;
    
    info!("Operator stopped.");

    return Ok(());
}
