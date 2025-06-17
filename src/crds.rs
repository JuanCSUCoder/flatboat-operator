use std::collections::HashMap;

use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(group = "flatboat.juancsu.coder", version = "v6", kind = "FlatboatWorkload", namespaced)]
#[kube(status = "FlatboatWorkloadStatus")]
pub struct FlatboatWorkloadSpec {
    image: String,
    launch: LaunchConfiguration,
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
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfiguration {
    package: String,
    executable: String,
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

#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(group = "flatboat.juancsu.coder", version = "v6", kind = "FlatboatBot", namespaced)]
#[kube(status = "FlatboatBotStatus")]
pub struct FlatboatBotSpec {
    node_name: String,
    capabilities: HashMap<String, String>,
    limitations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum FlatboatBotStatus {
    Idle,
    Busy,
    Offline,
}
