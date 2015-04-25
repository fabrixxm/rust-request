extern crate url;

use std::io::{Result, Error, ErrorKind};

pub enum Protocol {
    HTTP,
    HTTPS
}

pub struct Url {
    pub protocol: Protocol,
    pub host: String,
    pub port: u16,
    pub path: String
}

impl Url {
    pub fn new(url: &str) -> Result<Url> {
        let parsed_url = match url::Url::parse(url) {
            Ok(url) => url,
            Err(_) => {
                let err = Error::new(ErrorKind::InvalidInput,
                                     "Invalid URL.");
                return Err(err);
            }
        };

        let protocol = match &*parsed_url.scheme {
            "http" => Protocol::HTTP,
            "https" => Protocol::HTTPS,
            _ => {
                let err = Error::new(ErrorKind::InvalidInput,
                                     "Invalid URL.");
                return Err(err);
            }
        };
        
        let host = match parsed_url.domain() {
            Some(domain) => domain,
            None => {
                let err = Error::new(ErrorKind::InvalidInput,
                                     "Invalid URL.");
                return Err(err);
            }
        };

        let port = match parsed_url.port() {
            Some(port) => port,
            None => {
                let port = match protocol {
                    Protocol::HTTP => 80 as u16,
                    Protocol::HTTPS => 443 as u16
                };
                port
            }
        };

        let mut path = String::new();
        match parsed_url.path() {
            Some(p) => {
                for x in p.iter() {
                    path.push_str(&format!("/{}", x));
                }
            }
            None => {
                path.push_str("/");
            }
        };
        
        return Ok(Url {
            protocol: protocol,
            host: host.to_string(),
            port: port,
            path: path
        });
        
    }
}