use std::ptr::null;

use anyhow::Error;

#[allow(dead_code)]
pub struct Request {
    // to figure out
}

#[derive(Clone, Copy)]
pub struct Header {
    pub correlation_id: i32,
}

impl Header {
    pub fn new(correlation_id: &i32) -> Result<Self, Error> {
        let header = Header {
            correlation_id: *correlation_id,
        };

        Ok(header)
    }

    pub fn is_valid(&self) -> bool {
        return self.correlation_id.is_negative()
    }
}

#[allow(dead_code)]
pub struct Response {
    pub message_size: i32,
    pub header: Header,
    pub body: String,
}

impl Response {
    pub fn new(message_size: &i32, header: &Header, body: &String) -> Response {
        assert!(message_size.is_positive());
        assert!(!header.is_valid());
        assert!(!body.is_empty());

        Self {
            message_size: *message_size,
            header: *header,
            body: body.clone(),
        }
    }
}

#[allow(dead_code)]
impl Request {
    pub fn new() {
        // need to figure out...
    }
}