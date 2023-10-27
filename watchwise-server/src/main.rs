use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

static SERVER_ADDRESS: &str = "127.0.0.1:3005";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut app = Router::new().route("/", get(handle));

    app = app.layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let listener = net::TcpListener::bind(SERVER_ADDRESS).unwrap_or_else(|_| {
        panic!(
            "Enable to bind application to specific address: {}",
            SERVER_ADDRESS
        )
    });

    info!("Application started to listening on {}", SERVER_ADDRESS);
    axum::Server::from_tcp(listener)
        .expect("Unable to bind server to listener")
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn handle() -> impl IntoResponse {
    "Hello world"
}
