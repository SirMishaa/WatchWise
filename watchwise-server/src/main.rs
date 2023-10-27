use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net;

static SERVER_ADDRESS: &str = "127.0.0.1:3005";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle));

    let listener = net::TcpListener::bind(SERVER_ADDRESS).unwrap_or_else(|_| {
        panic!(
            "Enable to bind application to specific address: {}",
            SERVER_ADDRESS
        )
    });

    println!("Application started to listening on {}", SERVER_ADDRESS);
    axum::Server::from_tcp(listener)
        .expect("Unable to bind server to listener")
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn handle() -> impl IntoResponse {
    "Hello world"
}
