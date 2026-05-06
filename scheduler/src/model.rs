use serde::{Deserialize, Serialize}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentEntry {
    pub id: String,
    pub score: f64,
    pub pulls: u32,
}

#[derive(Debug, Deserialize)]
pub struct MetricsPayload {
    pub experiment_id: String,
    pub step: u32,
    pub val_accuracy: f64,
    pub loss: f64,
}

#[derive(Debug, Serialize)]
pub struct AllocationRespones {
    pub gpu_0: Option<String>,
    pub gpu_1: Option<String>,
}