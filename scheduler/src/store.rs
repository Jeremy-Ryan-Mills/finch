use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::ExperimentEntry;

pub type SharedStore = Arc<Mutex<HashMap<String, ExperimentEntry>>>

pub fn new_store() -> SharedStore {
    Arc::new(Mutex::new(HashMap::new()))
}