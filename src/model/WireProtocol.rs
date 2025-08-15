pub struct Request {
    // to figure out
}

pub struct Response {
    pub message_size: i32,
    pub header: String,
    pub body: String,
}

impl Response {
    pub fn new(message_size: i32, header: String, body: String) -> Response {
        assert!(message_size.is_positive());
        assert!(!header.is_empty());
        assert!(!body.is_empty());

        Self {
            message_size: message_size,
            header: header,
            body: body,
        }
    }
}

impl Request {
    pub fn new() {
        // need to figure out...
    }
}