use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum JokeError {
    ReqwestError(reqwest::Error),
    JsonParseError(serde_json::Error),
    EmptyResponse,
    MissingJokeField,
}

impl fmt::Display for JokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReqwestError(e) => write!(f, "Request failed: {}", e),
            Self::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            Self::EmptyResponse => write!(f, "Empty API response"),
            Self::MissingJokeField => write!(f, "Missing 'joke' field"),
        }
    }
}

impl From<reqwest::Error> for JokeError {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<serde_json::Error> for JokeError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonParseError(err)
    }
}

#[derive(Serialize, Deserialize)]
struct JokeApiResponse {
    joke: String,
}

pub async fn get_joke() -> Result<String, JokeError> {
    let response = reqwest::get("https://geek-jokes.sameerkumar.website/api?format=json")
        .await?
        .text()
        .await?;

    if response.is_empty() {
        return Err(JokeError::EmptyResponse);
    }

    let api_response: JokeApiResponse = serde_json::from_str(&response)?;
    Ok(api_response.joke)
}
