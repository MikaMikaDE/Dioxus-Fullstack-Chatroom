use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use axum::{
    Router, 
    middleware, 
    http::StatusCode,
    routing::{get,post},
    extract::{Path,Request},
    response::IntoResponse
};
const HOST_PORT: &str = concat!("0.0.0.0",":","3001");
const API_TOKEN: &str = "token";


async fn ping() -> impl IntoResponse { "Pong!" }

fn safe_segment(s: &str) -> Option<String> {
    let p = std::path::Path::new(s).file_name()?.to_str()?.to_string();
    (p != ".." && !p.is_empty()).then_some(p)
}

async fn write(Path((folder, filename)): Path<(String, String)>, body: String) -> impl IntoResponse {
    if let Err(e) = tokio::fs::create_dir_all(&folder).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Folder error: {e}"));
    }

    let (Some(folder), Some(filename)) = (safe_segment(&folder), safe_segment(&filename)) else {
        return (StatusCode::BAD_REQUEST, "bad path".to_string());
    };
    let path = format!("./data/{folder}/{filename}");
    let existing = tokio::fs::read_to_string(&path).await.unwrap_or_default();
    let combined = if existing.is_empty() { body } else { format!("{existing}\n{body}") };

    match tokio::fs::write(&path, &combined).await {
        Ok (_) => (StatusCode::OK, "saved".to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Write failed: {e}")),
    }
}
async fn read(Path((folder, filename)): Path<(String, String)>) -> impl IntoResponse {
    let (Some(folder), Some(filename)) = (safe_segment(&folder), safe_segment(&filename)) else {
        return (StatusCode::BAD_REQUEST, "bad path".to_string());
    };
    let path = format!("./data/{folder}/{filename}");
    match tokio::fs::read_to_string(&path).await {
        Ok (contents) => (StatusCode::OK, contents),
        Err(e       ) => (StatusCode::NOT_FOUND, format!("Not found: {e}")),
    }
}


async fn auth(req: Request, next: middleware::Next) -> impl IntoResponse {
    let is_authed = req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(     |v| v == format!("Bearer {API_TOKEN}"))
        .unwrap_or(false);

    if is_authed { next.run(req).await.into_response()                     } 
    else         { (StatusCode::UNAUTHORIZED, "bad token").into_response() }
}

#[tokio::main]
async fn main() {

    let public = Router::new()
        .route("/ping", get(ping))
        .route("/data/{folder}/{filename}", get(read));
    
    let private = Router::new()
        .route("/data/{folder}/{filename}", post(write))
        .layer(middleware::from_fn(auth));

    let app = Router::new()
        .merge(public)
        .merge(private)
        .fallback_service(ServeDir::new("../client/target/dx/client/release/web/public"))   // dx build output
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(HOST_PORT).await.expect("Server failed to start!");

    axum::serve(listener, app).await.unwrap();
}
