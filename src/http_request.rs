use anyhow::Error;
use derive_more::{Display, Error};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Display, Error)]
#[display(fmt = "The Http method {_0} is not supported")]
struct NotSupportedMethod(#[error(not(source))] String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: HttpProtocol,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
}

impl TryFrom<&str> for HttpMethod {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            method => Err(Error::from(NotSupportedMethod(method.to_string()))),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HttpProtocol {
    Http11,
    Http2,
}

impl TryFrom<&str> for HttpProtocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/1.1" => Ok(HttpProtocol::Http11),
            "HTTP/2" => Ok(HttpProtocol::Http2),
            protocol => Err(Error::from(NotSupportedMethod(protocol.to_string()))),
        }
    }
}

impl Display for HttpProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpProtocol::Http11 => write!(f, "HTTP/1.1"),
            HttpProtocol::Http2 => write!(f, "HTTP/2"),
        }
    }
}
