pub mod interactor;

use crate::interactor::media::OMDSearchResultMedia;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{de, Deserialize};
use std::str::FromStr;
use std::{fmt, net};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

static SERVER_ADDRESS: &str = "127.0.0.1:3005";

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    #[allow(dead_code)]
    query: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut app = Router::new()
        .route("/", get(handle))
        .route("/search", get(search_media));

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

async fn search_media(Query(params): Query<Params>) -> Json<Vec<OMDSearchResultMedia>> {
    let search_option = params.query.unwrap_or_default();
    let media_option =
        interactor::media::search_media(search_option, Some(interactor::media::MediaType::Movie))
            .await;
    let media = media_option.unwrap_or_else(Vec::new);

    Json(media)
}

/// Serde deserialization decorator to map empty Strings to None
fn empty_string_as_none<'de, Deserializer, Type>(
    deserializer: Deserializer,
) -> Result<Option<Type>, Deserializer::Error>
where
    Deserializer: serde::Deserializer<'de>,
    Type: FromStr,
    Type::Err: fmt::Display,
{
    let option = Option::<String>::deserialize(deserializer)?;
    match option.as_deref() {
        None | Some("") => Ok(None),
        Some(str) => FromStr::from_str(str).map_err(de::Error::custom).map(Some),
    }
}
