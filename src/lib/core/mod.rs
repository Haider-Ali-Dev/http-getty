pub mod methods;
use crate::error::{self, HttpGettyError};
use colored::Colorize;
use downcast_rs::{impl_downcast, Downcast};
use http::{method::Method, HeaderValue};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest::Client;
use std::{fs::File, io::Write};

use self::methods::SupportedMethods;

type RequesterResult = Result<Body, HttpGettyError>;

#[derive(Debug, Clone)]
pub enum Body {
    Json(String),
    Binary(Vec<u8>),
    Text(String),
}

impl ToString for Body {
    fn to_string(&self) -> String {
        match self {
            Self::Binary(_) => "".to_owned(),
            Self::Text(a) => a.to_owned(),
            Self::Json(a) => a.to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BodyType {
    Json,
    Binary,
    Text,
}

pub trait RawBodyType: Downcast {}
impl_downcast!(RawBodyType);
impl RawBodyType for String {}
impl RawBodyType for Vec<u8> {}

pub trait RawBody {
    fn raw(&self) -> Result<Box<dyn RawBodyType>, HttpGettyError>;
}

impl RawBody for Body {
    fn raw(&self) -> Result<Box<dyn RawBodyType>, HttpGettyError> {
        match self {
            Self::Text(v) => return Ok(Box::new(v.clone())),
            Self::Json(v) => return Ok(Box::new(v.clone())),
            Self::Binary(v) => return Ok(Box::new(v.clone())),
        }
    }
}

// impl RawBody<Vec<u8>> for Body {
//     fn raw(&self) -> Result<Vec<u8>, HttpGettyError> {
//         match self {
//             Self::Binary(v) => return Ok(v.clone()),
//             _ => Err(HttpGettyError::CannotConvertToRaw)
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Requester {
    pub url: String,
    pub headers: Option<String>,
    pub body: Option<Body>,
    pub method: Method,
    pub content_type: Option<BodyType>,
}

impl Requester {
    pub fn new(
        url: String,
        headers: Option<String>,
        body: Option<Body>,
        method: String,
    ) -> Result<Self, error::HttpGettyError> {
        let content_type = if let Some(v) = body.clone() {
            match v {
                Body::Binary(_) => Some(BodyType::Binary),
                Body::Json(_) => Some(BodyType::Json),
                Body::Text(_) => Some(BodyType::Text),
            }
        } else {
            None
        };
        let method = match method.to_lowercase().as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            _ => return Err(error::HttpGettyError::InvalidMethod),
        };
        Ok(Self {
            url,
            headers,
            body,
            method,
            content_type,
        })
    }

    pub async fn request(&self) -> RequesterResult {
        let method: SupportedMethods = self.clone().method.into();
        match method {
            SupportedMethods::Get => Ok(self.get().await?),
            SupportedMethods::Post => Ok(self.post().await?),
            SupportedMethods::Put => todo!(),
            SupportedMethods::Other => Err(HttpGettyError::InvalidMethod),
        }
    }

    async fn get(&self) -> RequesterResult {
        let client = Client::new();
        let mut response = client.get(self.clone().url).send().await?;
        if let Some(a) = response.headers().get("Content-Type") {
            match a.to_str().unwrap() {
                "application/json" | "text/html" | "text/plain" => {
                    return Ok(Body::Text(response.text().await?))
                }
                _ => {
                    println!("{}", "Found a binary object.".bold().green());
                    let bytes = response.bytes().await?.to_vec();
                    return Ok(Body::Binary(bytes));
                }
            }
        } else {
            return Err(HttpGettyError::UnSupportedResponse);
        }
    }

    async fn post_binary(&self) -> RequesterResult {
        todo!()
    }
    async fn post(&self) -> RequesterResult {
        if let None = self.body {
            return Err(HttpGettyError::NoBodyForPost);
        }
        let client = Client::new();
        let request = client.post(self.clone().url);
        let body: Box<dyn RawBodyType> = match self.clone().body.unwrap() {
            Body::Json(v) => self.clone().body.clone().unwrap().raw()?,
            Body::Text(v) => self.clone().body.clone().unwrap().raw()?,
            Body::Binary(v) => self.clone().body.clone().unwrap().raw()?,
        };

        let body_downcasted = match body.downcast::<String>() {
            Ok(v) => v,
            Err(_) => return self.post_binary().await,
        };

        let response = request.body(*body_downcasted.clone()).send().await?;
        if let Some(a) = response.headers().get("Content-Type") {
            match a.to_str().unwrap() {
                "text/html" | "text/plain" => return Ok(Body::Text(response.text().await?)),
                "application/json" => return Ok(Body::Json(response.text().await?)),
                _ => {
                    let stream = response.bytes().await?;
                    return Ok(Body::Binary(stream.to_vec()));
                }
            }
        } else {
            return Err(HttpGettyError::UnSupportedResponse);
        }
    }
}
