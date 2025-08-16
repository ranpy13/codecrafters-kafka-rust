use bytes::BufMut;
use crate::model::structs::{KafkaError, RespMessage};

pub async fn request_api_versions(request_api_version: i16, correlation_id: i32) -> Result<RespMessage, KafkaError> {
    match request_api_version {
        (0..5) => {
            let mut body: Vec<u8> = vec![];
            // 1st : Error Code :: i16 -> 0
            body.put_i16(0);
            // 2nd : COMPACT_ARRAY -> 1 (+1) (u8) && Api Key :: i16 -> 18
            body.put_u8(2);
            body.put_i16(18);
            // 3rd : Min Version :: i16 -> 0
            body.put_i16(0);
            // 4th : Max Version :: i16 -> 4
            body.put_i16(4);
            // TAG_FIELD IS U8 -> 0
            body.put_u8(0);
            // // 5th : Throttle Time ms :: i32 -> 50
            body.put_i32(0);
            // TAG_FIELD IS U8 -> 0
            body.put_u8(0);

            let mut resp = RespMessage::new_from_correlation_id(correlation_id);
            resp.change_body(body);

            Ok(resp)
        }
        _ => {
            Ok(RespMessage::new_error(correlation_id, 35))
        }
    }
}