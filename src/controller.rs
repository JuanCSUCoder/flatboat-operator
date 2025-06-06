use std::sync::Arc;

use kube::{config::Context, runtime::controller::Action, Client};

use crate::{apis::OperatorApis, crds};

#[derive(thiserror::Error, Debug)]
pub enum ReconcileError {
    #[error("Failed to reconcile workload: {0}")]
    ReconcileFailed(String),
}

pub async fn reconcile(
    _workload: Arc<crds::FlatboatWorkload>,
    _ctx: Arc<OperatorApis>
) -> Result<Action, ReconcileError> {
  return Ok(Action::await_change());
}

pub fn error_policy(
  _workload: Arc<crds::FlatboatWorkload>,
  _error: &ReconcileError,
  _ctx: Arc<OperatorApis>
) -> Action {
  Action::await_change()
}