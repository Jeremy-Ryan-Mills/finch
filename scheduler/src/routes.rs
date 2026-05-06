use uuid::Uuid;
use axum::{extract::{State, Path}, http::StatusCode, Json};
use crate::{engine, model::{AllocationResponse, ExperimentEntry, MetricsPayload}, store::SharedStore};

pub async fn register_experiment(
    State(store): State<SharedStore>,
    Json(payload): Json<ExperimentEntry>,
) -> StatusCode {
    let mut s = store.lock().unwrap();
    s.insert(payload.id.clone(), payload);
    StatusCode::CREATED
}

pub async fn deregister_experiment(
    State(store): State<SharedStore>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut s = store.lock().unwrap();
    match s.remove(&id) {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn get_allocations(
    State(store): State<SharedStore>,
) -> Json<AllocationResponse> {
    let s = store.lock().unwrap();
    let experiments: Vec<ExperimentEntry> = s.values().cloned().collect();
    let allocated = engine::allocate(&experiments, 2, 1.4);

    Json(AllocationResponse {
        gpu_0: allocated.get(0).cloned(),
        gpu_1: allocated.get(1).cloned(),
    })
}

pub async fn post_metrics(
    State(store): State<SharedStore>,
    Json(payload): Json<MetricsPayload>,
) -> StatusCode {
    let mut s = store.lock().unwrap();
    if let Some(exp) = s.get_mut(&payload.experiment_id) {
        exp.score = payload.val_accuracy;
        exp.pulls += 1;
    }
    StatusCode::OK
}