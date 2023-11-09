use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    io::Write,
    net::TcpStream,
};

use anyhow::Error;
use log::debug;
use maplit::hashmap;

use crate::http_request::HttpProtocol;

#[derive(Debug, Clone)]
pub struct HttpResponse<T> {
    pub status: HttpStatus,
    pub protocol: HttpProtocol,
    pub headers: HashMap<String, String>,
    pub data: T,
}

impl HttpResponse<String> {
    pub fn from_html(status: HttpStatus, protocol: HttpProtocol, data: impl ToString) -> Self {
        let bytes = data.to_string().into_bytes();

        let headers = hashmap! {
            "Content-Type".to_string() => "text/html".to_string(),
            "Content-Length".to_string() => bytes.len().to_string(),
        };

        Self {
            status,
            protocol,
            headers,
            data: data.to_string(),
        }
    }
}

impl<T: Display> HttpResponse<T> {
    fn status_headers_and_data(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        format!(
            "{} {}\r\n{headers}\r\n\r\n{}",
            self.protocol, self.status, self.data
        )
    }

    pub fn write(self, mut stream: &TcpStream) -> Result<(), Error> {
        let data = self.status_headers_and_data();
        debug!("{:?}", data);
        stream.write_all(data.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HttpStatus {
    Ok,
    NotFound,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
        }
    }
}
