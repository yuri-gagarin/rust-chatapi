use chrono;
use crate::MessageData;

pub fn generate_mock_messages(num_of_messages: usize) -> Vec<MessageData> {
    let mut message_data_vec: Vec<MessageData> = Vec::with_capacity(num_of_messages);
    for i in 0..num_of_messages {
        println!("Line at {}", i);
        let mock_message: MessageData = MessageData { 
          id: 12234, 
          date: chrono::Utc::now().to_string(),
          sender: "John Smith".to_string(), 
          receiver: "Bob Doe".to_string(), 
          data: "Mock message data is here".to_string(), 
          read: true 
        };
        message_data_vec.push(mock_message);
    };
  
    message_data_vec
}
pub fn generate_single_message() -> MessageData {
    let mock_message: MessageData = MessageData { 
      id: 12234, 
      date: chrono::Utc::now().to_string(),
      sender: "John Smith".to_string(), 
      receiver: "Bob Doe".to_string(), 
      data: "Mock message data is here".to_string(), 
      read: true 
    };
    mock_message
}

pub fn generate_rand_id() -> i64 {
    chrono::Utc::now().timestamp()
}
pub fn generate_date_now() -> String {
    chrono::Utc::now().to_string()
}