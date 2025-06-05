mod k8s;
mod crds;

use tracing::{info, debug};

use crate::k8s::ApisInitError;

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
    let apis = k8s::initialize_apis().await?;
    debug!("Kubernetes APIs initialized successfully");

    return Ok(());
}
