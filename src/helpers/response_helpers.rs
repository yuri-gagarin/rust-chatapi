use crate::MessageData;

//use crate::MessageData;
use super::super::routes::types::ResponseData;

pub enum SingleMsgRes {
    QueriedRes,
    CreatedRes,
    EditedRes,
    DeletedRes,
}
pub struct MessageResponses {}

impl MessageResponses {
    pub fn error_response(response_message: String, total_messages: u32, error_messages: Vec<String>) -> ResponseData {
        let response = ResponseData {
            response_message: response_message,
            messages_list: None,
            message: None,
            new_message: None,
            edited_message: None,
            deleted_message: None,
            total_messages: total_messages,
            error_messages: Some(error_messages),
        };
        response
    }
    pub fn all_messages_response(response_message: String, total_messages: u32, messages: Vec<MessageData>) -> ResponseData {
        let response = ResponseData {
            response_message,
            messages_list: Some(messages),
            message: None,
            new_message: None,
            edited_message: None,
            deleted_message: None,
            total_messages: total_messages,
            error_messages: None,
        };
        response
    }
    pub fn single_message_response(response_message: String, total_messages: u32, 
                                   message_data: MessageData, response_type:SingleMsgRes) -> ResponseData {
        let mut response = ResponseData {
            response_message,
            messages_list: None,
            message: None,
            new_message: None,
            edited_message: None,
            deleted_message: None,
            total_messages: total_messages,
            error_messages: None,
        };
        match response_type {
            SingleMsgRes::QueriedRes => {
                response.message = Some(message_data);
                response
            }
            SingleMsgRes::CreatedRes => {
                response.new_message = Some(message_data);
                response.total_messages += 1;
                response
            }
            SingleMsgRes::EditedRes => {
                response.edited_message = Some(message_data);
                response
            }   
            SingleMsgRes::DeletedRes => {
                response.deleted_message = Some(message_data);
                response.total_messages -= 1;
                response
            }
        }
    }
}