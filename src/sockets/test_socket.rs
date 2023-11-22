use actix::{Actor, AsyncContext, fut};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;
use super::messages::{Connect, Disconnect};

const BEAT_INTERVAL: Duration = Duration::from_secs(5);
const TIMEOUT_INTERVAL: Duration = Duration::from_secs(10);

pub struct TestWSConnection {
    room_id: Uuid,
    lobby_address: Uuid,
    heartbeat: Instant,
    connection_id: Uuid
}

impl TestWSConnection {
    pub fn new(room_id: Uuid, lobby: Uuid) -> TestWSConnection {
        TestWSConnection { 
            room_id: Uuid::new_v4(), 
            lobby_address: Uuid::new_v4(), 
            heartbeat: Instant::now(), 
            connection_id: Uuid::new_v4()
        }
    }
}

impl Actor for TestWSConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        // stop 
        let address = ctx.address();
        self.lobby_address.send(Connect {
            address: address.recipient(),
            lobby_id: self.room_id,
            self_id: self.connection_id
        })
        .into_actor(self)
        .then(|res, _, ctx| {
            match res {
                Ok(_res) => (),
                _ => ctx.stop(),
            }
            fut::ready(())
        })
        .wait(ctx)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::prelude::Running {
        self.lobby_address.do_send(Disconnect {
            self_id: self.connection_id,
            room_id: self.room_id
        });
        actix::prelude::Running::Stop
    }
}