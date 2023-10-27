use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct OMDSearchParams {
    /// Movie title to search for
    #[serde(rename = "s")]
    search: String,

    /// Type of result to return
    #[serde(rename = "type")]
    type_: Option<MediaType>,

    /// Year of release
    #[serde(rename = "y")]
    year: Option<String>,

    /// Page number to return
    page: Option<u8>,

    /// API version
    #[serde(rename = "v")]
    version: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "series")]
    Series,
    #[serde(rename = "episode")]
    Episode,
}

static OMD_URL: &str = "https://www.omdbapi.com/?apikey=8e7eb08a";

pub async fn search_media(search: String, media_type: Option<MediaType>) -> String {
    let params = OMDSearchParams {
        search: search.clone(),
        type_: media_type,
        year: None,
        page: None,
        version: None,
    };

    // Todo: Find a safer way to build the URL, that handle case where query params are empty
    let url = format!(
        "{}&{}",
        OMD_URL,
        serde_urlencoded::to_string(params).expect("Unable to encode params")
    );

    info!("Searching for media on OMD database with term: {}", search);
    url
}
