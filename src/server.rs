use anyhow::Error;
use derive_more::{Display, Error};
use log::debug;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::http_request::{HttpMethod, HttpProtocol, HttpRequest};

#[derive(Debug, Display, Error)]
#[display(fmt = "Request is empty")]
struct EmptyRequest;

#[derive(Debug, Display, Error)]
#[display(fmt = "Http method is missing")]
struct MissingHttpMethod;

#[derive(Debug, Display, Error)]
#[display(fmt = "Request path is missing")]
struct MissingPath;

#[derive(Debug, Display, Error)]
#[display(fmt = "Request protocol version is missing")]
struct MissingHttpProtocol;

#[derive(Debug, Display, Error)]
#[display(fmt = "Header name is missing")]
struct MissingHeaderName;

#[derive(Debug, Display, Error)]
#[display(fmt = "Header {_0} is missing value")]
struct MissingHeaderValue(#[error(not(source))] String);

pub fn parse_request(mut stream: &TcpStream) -> Result<HttpRequest, Error> {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next();

    if request_line.is_none() {
        return Err(Error::from(EmptyRequest));
    }

    let request_line = request_line.unwrap()?;
    let mut parts = request_line.split_whitespace();

    let method = HttpMethod::try_from(parts.next().ok_or(MissingHttpMethod)?)?;
    let path: String = parts.next().ok_or(MissingPath).map(Into::into)?;
    let protocol = HttpProtocol::try_from(parts.next().ok_or(MissingHttpProtocol)?)?;

    let mut headers = HashMap::new();

    loop {
        let header_line = lines.next();

        if header_line.is_none() {
            break;
        }

        let header_line = header_line.unwrap()?;

        debug!("{header_line}");

        if header_line.is_empty() {
            break;
        }

        let mut comps = header_line.split(':');
        let key = comps.next().ok_or(MissingHeaderName)?;
        let value = comps
            .next()
            .ok_or(MissingHeaderValue(key.to_string()))?
            .trim();

        headers.insert(key.to_string(), value.to_string());
    }

    Ok(HttpRequest {
        method,
        path,
        protocol,
        headers,
    })
}
