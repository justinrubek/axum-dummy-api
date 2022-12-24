use axum::{
    routing::{get, post},
    Router, http::{Request, Response}, body::Body, extract::Path
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get the port to listen on from the environment, default 8000
    let port = std::env::var("PORT")
        .ok()
        .and_then(|it| it.parse().ok())
        .unwrap_or(8000);

    let app = Router::new()
        .route("/", get(hello))
        .route("/echo/", post(echo))
        .route("/echo/:name", get(echo_name))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn echo(req: Request<Body>) -> Response<Body> {
    Response::new(req.into_body())
}

async fn echo_name(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

async fn health() -> &'static str {
    "OK"
}
