use std::env;
use axum::{routing::post, Router, Json};
use axum::body::Bytes;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };
    let app = Router::new()
        .route("/echo", post(echo))
        .route("/queue", post(queue));

    axum::Server::bind(&format!("127.0.0.1:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn queue(input: Bytes) -> (StatusCode, Json<QueueResponse>) {
    let response = QueueResponse {
        logs: vec![
            format!("Got request JSON: {}", String::from_utf8_lossy(input.as_ref()))
        ],
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct QueueResponse {
    #[serde(rename = "Logs")]
    pub logs: Vec<String>,
}


async fn echo(Json(request): Json<FunctionRequest>) -> (StatusCode, Json<FunctionResponse>) {
    let inner_echo_request: InnerRequest = serde_json::from_str(&request.data.req.body).unwrap();
    let inner_echo_response = serde_json::to_string(&EchoResponse {
        resp: inner_echo_request.req,
    }).unwrap();

    let response = FunctionResponse {
        outputs: Outputs {
            res: Res {
                body: inner_echo_response
            }
        },
        logs: vec![]
    };

    (StatusCode::OK, Json(response))
}

//
// Function request deserialization
//

#[derive(Deserialize)]
struct FunctionRequest {
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Deserialize)]
struct Data {
    pub req: Req,
}

#[derive(Deserialize)]
struct Req {
    #[serde(rename = "Body")]
    pub body: String,
}

//
// Function response deserialization
//

#[derive(Serialize)]
struct FunctionResponse {
    #[serde(rename = "Outputs")]
    pub outputs: Outputs,
    #[serde(rename = "Logs")]
    pub logs: Vec<String>,
}

#[derive(Serialize)]
struct Outputs {
    pub res: Res,
}

#[derive(Serialize)]
struct Res {
    pub body: String,
}

//
// Inner HTTP request and response
//

#[derive(Deserialize)]
struct InnerRequest {
    req: String,
}

#[derive(Serialize)]
struct EchoResponse {
    resp: String,
}
