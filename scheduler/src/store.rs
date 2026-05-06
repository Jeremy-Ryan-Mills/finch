use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::model::ExperimentEntry;

pub type SharedStore = Arc<Mutex<HashMap<Uuid, ExperimentEntry>>>;

pub fn new_store() -> SharedStore {
    Arc::new(Mutex::new(HashMap::new()))
}