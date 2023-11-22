use actix::prelude::{Message, Recipient};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebSocketMsg {
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub address: Recipient<WebSocketMsg>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub self_id: Uuid,
}