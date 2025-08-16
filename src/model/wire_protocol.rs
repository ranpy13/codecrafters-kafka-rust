#![allow(dead_code)]

use std::ptr::null;

use anyhow::Error;

#[derive(Clone)]
pub struct Header<'a> {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<String>,
    pub tag_buffer: Vec<&'a str> 
}

impl<'a> Header<'a> {
    pub fn new(reqeust_api_key: &i16, request_api_version: &i16, correlation_id: &i32, client_id: &Option<String>, tag_buffer: &Vec<&'a str>) -> Result<Self, Error> {
        assert!(reqeust_api_key.is_positive());
        // assert!(0 < *request_api_version && *reqeust_api_key <= 4);
        assert!(correlation_id.is_positive());

        let header = Header {
            request_api_key: *reqeust_api_key,
            request_api_version: *request_api_version,
            correlation_id: *correlation_id,
            client_id: client_id.clone(),
            tag_buffer: tag_buffer.clone(),
        };

        Ok(header)
    }

    pub fn is_valid(&self) -> bool {
        return self.correlation_id.is_negative()
    }
}

#[derive(Clone, Copy)]
pub struct ApiVersionArray {
    pub api_key: i16,
    pub min_version: i16, 
    pub max_version: i16,
    pub tag_buffer: i8,
}

#[derive(Clone, Copy)]
pub struct Body {
    pub error_code: i16,
    pub array_length: i8,
    pub api_version_array: ApiVersionArray,
    pub throttle_time: i32,
    pub tag_buffer: i8,
}

pub struct Request<'a> {
    message_size: i32,
    header: Header<'a>,
    body: String    
}

impl ApiVersionArray {
    pub fn new(api_key: &i16, min_version: &i16, max_version: &i16, tag_buffer: &i8) -> Self {
        Self {
            api_key: *api_key,
            min_version: *min_version,
            max_version: *max_version,
            tag_buffer: *tag_buffer,
        }
    }
}

impl Body {
    pub fn new(error_code: &i16, array_length: &i8, api_version_array: &ApiVersionArray, throttle_time: &i32, tag_buffer: &i8) -> Self {
        Self { 
            error_code: *error_code, 
            array_length: *array_length, 
            api_version_array: *api_version_array, 
            throttle_time: *throttle_time, 
            tag_buffer: *tag_buffer 
        }
    }
}

pub struct Response<'a> {
    pub message_size: i32,
    pub header: Header<'a>,
    pub body: Body,
}

impl<'a> Response<'a> {
    pub fn new(message_size: &i32, header: &Header<'a>, body: &Body) -> Response<'a> {
        assert!(message_size.is_positive());
        assert!(!header.is_valid());
        // assert!(!body.is_empty());

        let response = Response {
            message_size: *message_size,
            header: header.clone(),
            body: body.clone(),
        };

        response
    }
}

impl<'a> Request<'a> {
    pub fn new(message_size: &i32, header: &Header<'a>, body: &String) -> Self {
        assert!(message_size.is_positive());
        assert!(!header.is_valid());
        assert!(!body.is_empty());

        Request { 
            message_size: *message_size, 
            header: header.clone(), 
            body: body.clone() 
        }
    }
}