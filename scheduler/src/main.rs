use axum::{routing::{get, post}, Router};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


mod engine;
mod models;
mod routes;
mod store;

// generated once at startup and used with every log! call
static LOG_FILE: std::sync::OnceLock<String> = std::sync::OnceLock::new();


#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let msg = format!("[log]({ts}) {}", format!($($arg)*));
        println!("{}", msg);

        let path = LOG_FILE.get_or_init(init_log_file);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        writeln!(file, "{}", msg).unwrap();
    }};
}

fn init_log_file() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()                        
        .unwrap()
        .join("logs");

    fs::create_dir_all(&log_dir).unwrap();

    log_dir
        .join(format!("scheduler_{}.log", ts))
        .to_string_lossy()
        .to_string()
}

#[tokio::main]
async fn main() {
    // start the log file for this run
    LOG_FILE.get_or_init(init_log_file);

    log!("Scheduler starting")

    let store = store::new_store();

    // setup crud endpoints for ipc with python
    let app = Router::new()
        .route("/experiments", post(routes::register_experiment))
        .route("/allocations", get(routes::get_allocations))
        .route("/metrics", post(routes::post_metrics))
        .with_state(store);

    // bind to port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    log!("Scheduler running on port 8000");
    axum::serve(listener, app).await.unwrap();
}