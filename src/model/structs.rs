use bytes::BufMut;

#[derive(thiserror::Error, Debug)]
pub enum KafkaError {
    #[error("failed to read the key file")]
    FileReadError(#[source] std::io::Error),

    #[error("`{0}`")]
    StringError(String),

    #[error("failed to delete the key file")]
    FileDeleteError(#[source] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct RespMessage {
    message_size: i32,
    header: Option<i32>,
    body: Option<Vec<u8>>,
}

pub trait Streamable {
    fn to_hex(&self) -> Vec<u8>;
}

pub trait Builder {
    fn new() -> Self;
}

impl Streamable for RespMessage {
    fn to_hex(&self) -> Vec<u8> {
        let mut data = vec![];
        let mut ans = vec![];
        if let Some(header) = self.header {
            data.extend_from_slice(&header.to_be_bytes());
        }
        if let Some(body) = self.body.clone() {
            data.extend_from_slice(&body);
        }
        if let Ok(message_size) = i32::try_from(data.len()) {
            ans.put_i32(message_size);
        } else {
            ans.put_i32(0);
        }
        ans.append(&mut data);

        ans
    }
}

impl RespMessage {
    pub fn change_message_size(&mut self, new_size: i32) {
        self.message_size = new_size;
    }

    pub fn change_correlation_id(&mut self, new_correlation_id: i32) {
        self.header = Some(new_correlation_id);
    }

    pub fn change_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    pub fn body_len(&self) -> usize {
        if let Some(body) = self.body.clone() {
            body.len()
        } else {
            0
        }
    }

    pub fn new_error(correlation_id: i32, error_number: i16) -> Self {
        RespMessage {
            message_size: 6,
            header: Some(correlation_id),
            body: Some(error_number.to_be_bytes().to_vec()),
        }
    }

    pub fn new_from_correlation_id(correlation_id: i32) -> Self {
        RespMessage {
            message_size: 4,
            header: Some(correlation_id),
            body: None,
        }
    }
}

impl Builder for RespMessage {
    fn new() -> Self {
        RespMessage {
            message_size: 0,
            header: None,
            body: None,
        }
    }
}