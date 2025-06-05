use tracing::{info, debug};

#[derive(thiserror::Error, Debug)]
enum OperatorError {
    #[error("Kubernetes client error: {0}")]
    KubeError(#[from] kube::Error),
}

type ProgramResult = Result<(), OperatorError>;

#[tokio::main]
async fn main() -> ProgramResult {
    // 1. Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();
    info!("Starting flatboat operator...");

    // 2. Create Kubernetes client
    let client = kube::Client::try_default().await?;
    debug!("Kubernetes client created successfully");

    return Ok(());
}
