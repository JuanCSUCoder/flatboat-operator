use std::sync::Arc;

use kube::{runtime::controller::Action, ResourceExt};

use crate::{apis::OperatorApis, crds};

#[derive(thiserror::Error, Debug)]
pub enum ReconcileError {
    #[error("Failed to patch the workload status: {0}")]
    WorkloadPatchFailed(#[from] kube::Error),
}

pub async fn reconcile(
    workload: Arc<crds::FlatboatWorkload>,
    ctx: Arc<OperatorApis>
) -> Result<Action, ReconcileError> {
  let name = workload.name_any();
  let namespace = workload.namespace().unwrap_or_default();

  ctx.workload
    .patch_status(
      &name,
      &kube::api::PatchParams::default(),
      &kube::api::Patch::Apply(
        serde_json::json!({
          "status": crds::FlatboatWorkloadStatus::Reallocating
        })
      )
    )
    .await?;

  return Ok(Action::await_change());
}

pub fn error_policy(
  _workload: Arc<crds::FlatboatWorkload>,
  _error: &ReconcileError,
  _ctx: Arc<OperatorApis>
) -> Action {
  Action::await_change()
}