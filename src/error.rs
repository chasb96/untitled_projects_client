use std::fmt::Display;

use prost::DecodeError;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Status(StatusCode),
    ProtobufDecode(DecodeError),
    Json(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_status() {
            Self::from(value.status().unwrap())
        } else {
            Self::Http(value)
        }
    }
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        Self::Status(value)
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::ProtobufDecode(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(http) => write!(f, "Error calling host: {}", http),
            Error::Status(status) => write!(f, "Host returned an error: {}", status),
            Error::ProtobufDecode(decode) => write!(f, "Error decoding response: {}", decode),
            Error::Json(json) => write!(f, "Error in JSON: {}", json),
        }
    }
}

impl std::error::Error for Error { }