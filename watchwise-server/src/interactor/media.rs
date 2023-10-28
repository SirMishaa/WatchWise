use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io;
use tracing::{debug, error, info};

type BoxError = Box<dyn std::error::Error>;

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

// Todo: Deserialize into more precise type for some field, like year and poster url
#[derive(Debug, Serialize, Deserialize)]
pub struct OMDSearchResultMedia {
    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Year")]
    year: String,

    #[serde(rename = "imdbID")]
    imdb_id: String,

    #[serde(rename = "Type")]
    type_: MediaType,

    #[serde(rename = "Poster")]
    poster: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OMDSearchResult {
    #[serde(rename = "Search")]
    search: Vec<OMDSearchResultMedia>,
    #[serde(rename = "totalResults")]
    total_results: String,
    #[serde(rename = "Response")]
    response: String,
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

pub async fn search_media(
    search: String,
    media_type: Option<MediaType>,
) -> Option<Vec<OMDSearchResultMedia>> {
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
    debug!("OMD API URL: {}", url);

    // Todo: Handle case when OMD API do not find any result. Search field will not be there and the response code will be 200 with "Response" field set to "False" 😅
    let result: Result<OMDSearchResult, BoxError> = fetch_data(&url).await;
    match result {
        Ok(response) => {
            if response.response == "True" {
                Some(response.search)
            } else {
                None
            }
        }
        Err(error) => {
            error!("Unable to fetch data from OMD API. Error: {}", error);
            None
        }
    }
}

async fn fetch_data<TargetType: DeserializeOwned>(url: &str) -> Result<TargetType, BoxError> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        let error_message = format!("Request failed with status code: {}", response.status());
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            error_message,
        )));
    }

    let result = response.json::<TargetType>().await;
    match result {
        Ok(parsed_response) => Ok(parsed_response),
        Err(error) => {
            let error_message = format!("Unable to parse response. Error: {}", error);
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                error_message,
            )))
        }
    }
}
