use chrono;
use rand::distributions::{Alphanumeric, DistString};
use crate::MessageData;

/* Mock message data */
pub fn generate_mock_messages(num_of_messages: usize) -> Vec<MessageData> {
    let mut message_data_vec: Vec<MessageData> = Vec::with_capacity(num_of_messages);
    let mut mock_message_id: i64 = 1000;
    for i in 0..num_of_messages {
        println!("Line at {}", i);
        let mock_message = generate_single_message(mock_message_id);
        message_data_vec.push(mock_message);
        mock_message_id += 1;
    };
  
    message_data_vec
}
pub fn generate_single_message(id: i64) -> MessageData {
    let mock_message: MessageData = MessageData { 
      id,
      date: chrono::Utc::now().to_string(),
      sender: "John Smith".to_string(), 
      receiver: "Bob Doe".to_string(), 
      data: generate_random_string(15), 
      read: true 
    };
    mock_message
}

/* Create a single message */
pub fn create_new_message(id: i64, sender: String, receiver: String, data: String) -> MessageData {
    return MessageData {
        id,
        date: generate_date_now(),
        sender,
        receiver,
        data,
        read: true
    };
}

/* random data */
/* 
pub fn generate_rand_id() -> i64 {
    chrono::Utc::now().timestamp()
}
*/
pub fn generate_date_now() -> String {
    chrono::Utc::now().to_string()
}
pub fn generate_random_string(string_len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(),string_len)
}


pub fn invalid_id_params(param: String) -> (bool, i64) {
    match param.parse::<i64>() {
        Ok(number) => {
            (false, number)
        }
        Err(_error) => {
            (true, 0)
        }
    }
}
