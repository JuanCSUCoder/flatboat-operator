use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[kube(group = "flatboat.juancsu.coder", version = "v0.6.0", kind = "FlatboatWorkload", namespaced)]
#[kube(status = "FlatboatWorkloadStatus")]
pub struct FlatboatWorkloadSpec {
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum FlatboatWorkloadStatus {
    Running,
    Finished,
    Reallocating,
    FailLoopBackoff,
}
