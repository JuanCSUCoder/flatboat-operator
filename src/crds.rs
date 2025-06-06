use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[kube(group = "flatboat.juancsu.coder", version = "v0.6.0", kind = "FlatboatWorkload", namespaced)]
#[kube(status = "FlatboatWorkloadStatus")]
pub struct FlatboatWorkloadSpec {
    image: String,
    launch_executable: String,
    args: Vec<String>,
    #[schemars(default)]
    #[serde(default = "default_node_selector")]
    runs_on: FlatboatWorkloadNodeSelector
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum FlatboatWorkloadNodeSelector {
    Robot,
    GPU,
    CPU,
}

impl Default for FlatboatWorkloadNodeSelector {
    fn default() -> Self {
        FlatboatWorkloadNodeSelector::CPU
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum FlatboatWorkloadStatus {
    Running,
    Finished,
    Reallocating,
    FailLoopBackoff,
}

fn default_node_selector() -> FlatboatWorkloadNodeSelector {
    FlatboatWorkloadNodeSelector::CPU
}
