//use crate::MessageData;
use super::super::routes::types::ResponseData;

pub enum MsgOkRes {
    OkRes,
    CreatedRes,
    EditedRes,
}
pub struct MessageResponses {}

impl MessageResponses {
    pub fn error_response(response_message: String, error_messages: &Vec<String>) -> ResponseData {
        let response = ResponseData {
            response_message: response_message,
            messages: None,
            new_message: None,
            edited_message: None,
            deleted_message: None,
            total_messages: None,
            error_messages: Some(*error_messages),
        };
        response
    }
}