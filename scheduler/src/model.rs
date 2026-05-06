use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentEntry {
    pub id: Uuid,
    pub score: f64,
    pub pulls: u32,
}

#[derive(Debug, Deserialize)]
pub struct MetricsPayload {
    pub experiment_id: Uuid,
    pub step: u32,
    pub val_accuracy: f64,
    pub loss: f64,
}

#[derive(Debug, Serialize)]
pub struct AllocationResponse {
    //pub gpu_0: Option<Uuid>,
    //pub gpu_1: Option<Uuid>,
    pub gpus: Vec<Uuid>,
}