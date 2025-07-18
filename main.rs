use axum::{extract::State, Json};
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::default();

    let app = Router::new()
        .route("/get", get(get_handler))
        .route("/insert", get(insert_handler))
        .route("/update", get(update_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/*****************************/
/********** Handlers *********/
pub async fn get_handler(State(state): State<AppState>) -> Json<Value> {
    let counters: tokio::sync::MutexGuard<'_, Vec<Counter>> = state.db.lock().await;

    let counters: Vec<Counter> = counters.clone();
    println!("GET Counters: {:?}", counters);

    return Json(json!({ "counters": counters }));
}

pub async fn insert_handler(State(state): State<AppState>) -> Json<Value> {
    let mut counters: tokio::sync::MutexGuard<'_, Vec<Counter>> = state.db.lock().await;

    counters.insert(0, Counter { count: 5 });

    let counters: Vec<Counter> = counters.clone();

    println!("INSERT Counters: {:?}", counters);
    return Json(json!({ "counters": counters }));
}

pub async fn update_handler(State(state): State<AppState>) -> Json<Value> {
    let mut counters: tokio::sync::MutexGuard<'_, Vec<Counter>> = state.db.lock().await;

    // there is also iter_mut() for interating, last_mut() to get the last and etc.
    let Some(counter) = counters.first_mut() else {
        return Json(json!({ "error": "No counter found to update." }));
    };

    counter.count += 1;

    let counters: Vec<Counter> = counters.clone();

    println!("UDPATE Counters: {:?}", counters);
    return Json(json!({ "counters": counters }));
}

/**************************/
/********** Model *********/
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct AppState {
    pub db: Arc<Mutex<Vec<Counter>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Counter {
    pub count: i32,
}
