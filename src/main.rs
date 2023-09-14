use std::env;
use axum::{routing::post, Router, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };
    let app = Router::new().route("/api/echo", post(echo));

    axum::Server::bind(&format!("127.0.0.1:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn echo(Json(payload): Json<EchoRequest>) -> (StatusCode, Json<EchoResponse>) {
    let echo = EchoResponse {
        resp: payload.req
    };

    (StatusCode::OK, Json(echo))
}

#[derive(Deserialize)]
struct EchoRequest {
    req: String,
}

#[derive(Serialize)]
struct EchoResponse {
    resp: String,
}
