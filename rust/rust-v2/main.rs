use axum::{routing::get, Router, response::IntoResponse};
use std::{env, fs};
use std::net::SocketAddr;
use ddprof::ProfilerGuardBuilder;

async fn root_handler() -> impl IntoResponse {
    let app_version = env::var("APP_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let count = calc_target_logic(&app_version);
    println!("count: {}", count);
    "Hello World!"
}

fn calc_target_logic(app_version: &str) -> usize {
    let dummy_data = read("./data/input.txt").unwrap_or_default();
    count(&dummy_data, app_version)
}

fn read(filename: &str) -> Option<Vec<u8>> {
    let content = fs::read_to_string(filename).ok()?;
    let mut dummy_data = Vec::new();
    for c in content.chars() {
        if let Some(d) = c.to_digit(10) {
            if d == 0 || d == 1 {
                dummy_data.push(d as u8);
            } else {
                return None;
            }
        }
    }
    Some(dummy_data)
}

fn count(dummy_data: &[u8], _app_version: &str) -> usize {
    let mut data = dummy_data.to_vec();
    let n = data.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if data[j] > data[j+1] {
                data.swap(j, j+1);
            }
        }
    }
    let index = data.iter().position(|&x| x == 1).unwrap_or(n);
    println!("index: {}", index);
    n - index
}

#[tokio::main]
async fn main() {
    // Datadog Profiler 初期化
    let _guard = ProfilerGuardBuilder::default().build().ok();

    let app = Router::new().route("/", get(root_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
} 