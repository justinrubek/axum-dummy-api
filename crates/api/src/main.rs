use axum::{
    routing::{get, post},
    Router, http::{Request, Response}, body::Body, extract::Path
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/echo/", post(echo))
        .route("/echo/:name", get(echo_name));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
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
