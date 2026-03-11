use axum::{extract::Query, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize)]
struct TimeResponse {
    timestamp: i64,
    iso8601: String,
    date: String,
    time: String,
    timezone: String,
}

#[derive(Deserialize)]
struct TimeQuery {
    tz: Option<String>,
}

async fn get_time(Query(params): Query<TimeQuery>) -> Json<TimeResponse> {
    let tz_str = params.tz.unwrap_or_else(|| "Asia/Shanghai".to_string());
    let tz = Tz::from_str(&tz_str).unwrap_or(chrono_tz::Asia::Shanghai);

    let now_utc: DateTime<Utc> = Utc::now();
    let now_tz: DateTime<Tz> = now_utc.with_timezone(&tz);

    Json(TimeResponse {
        timestamp: now_utc.timestamp_millis(),
        iso8601: now_tz.to_rfc3339(),
        date: now_tz.format("%Y-%m-%d").to_string(),
        time: now_tz.format("%H:%M:%S").to_string(),
        timezone: tz_str,
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/time", get(get_time));

    let addr = "0.0.0.0:8080";
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
