use axum::{routing::post, Router, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/echo", post(echo));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
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
