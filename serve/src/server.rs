use actix::prelude::*;
use message::{FromClientMessage, FromServerMessage};
use std::collections::HashMap;

type AddrWS = Addr<super::my_ws::MyWs>;

#[derive(Debug)]
pub struct Server {
    addrs: HashMap<usize, AddrWS>,
    count: usize,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub ctx: AddrWS,
}

#[derive(Message)]
#[rtype(usize)]
pub struct SrvMessage {
    pub msg: String,
}
#[derive(Message)]
#[rtype(usize)]
pub struct EntryMessage {
    pub payload: FromClientMessage,
    pub client: super::my_ws::MyWs,
}

impl Server {
    pub fn new() -> Self {
        Server {
            addrs: HashMap::new(),
            count: 0,
        }
    }

    fn send_message_to_chat_box(&self, txt: String) {
        let message_chat = if txt == "reload" {
            FromServerMessage::Reload
        } else {
            FromServerMessage::Chat(txt)
        };
        for (_id, ctx) in self.addrs.iter() {
            ctx.do_send(SrvMessage {
                msg: message_chat.to_serialized(),
            });
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<EntryMessage> for Server {
    type Result = usize;

    fn handle(&mut self, msg: EntryMessage, _ctx: &mut Context<Self>) -> usize {
        match msg.payload {
            FromClientMessage::Login(txt) => {
                msg.client.set_login(Some(txt));
            }
            FromClientMessage::ChatMsg(txt) => {

                let login = if let Some(name) = msg.client.get_login() {
                    name
                } else {
                    "nc".to_string()
                };
                self.send_message_to_chat_box(format!("{login} : {txt}"));
            }
        }
        1
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler<Connect> for Server {
    type Result = usize;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> usize {
        let id = self.count;
        self.addrs.insert(id, msg.ctx);
        self.count += 1;
        id
    }
}
