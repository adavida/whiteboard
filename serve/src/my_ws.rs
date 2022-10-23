use std::sync::{Arc, Mutex};

use crate::server::EntryMessage;

use super::server;
use actix::prelude::*;
use actix_web_actors::ws;

type AddrServer = Addr<server::Server>;

#[derive(Debug)]
pub struct ClientData {
    pub login: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MyWs {
    pub addr: AddrServer,
    pub id: usize,
    pub client_data: Arc<Mutex<ClientData>>,
}

impl Drop for ClientData {
    fn drop(&mut self) {
        println!("drop");
    }
}

impl MyWs {
    pub fn new(addr: AddrServer) -> Self {
        let client_data = Arc::new(Mutex::new(ClientData { login: None }));
        Self {
            addr,
            id: 0,
            client_data,
        }
    }

    pub fn set_login(&self, login: Option<String>) {
        let mut c = self.client_data.lock().unwrap();
        c.login = login;
    }

    pub fn get_login(&self) -> Option<String> {
        let c = self.client_data.lock().unwrap();
        c.login.clone()
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is alive");
        let _res = self
            .addr
            .send(server::Connect {
                ctx: _ctx.address(),
            })
            .into_actor(self)
            .then(|res, _ws, _ctx| {
                if res.is_err() {
                    println!("err");
                }
                fut::ready(())
            })
            .wait(_ctx);
        _ctx.text("hello word!");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let message = message::FromClientMessage::from_serialized(text.to_string());
                if let Ok(payload) = message {
                    self.addr
                        .send(EntryMessage {
                            payload,
                            client: self.clone(),
                        })
                        .into_actor(self)
                        .then(|res, _act, ctx| {
                            match res {
                                Ok(_res) => {}
                                _ => ctx.stop(),
                            };
                            fut::ready(())
                        })
                        .wait(ctx);
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

impl Handler<server::SrvMessage> for MyWs {
    type Result = usize;

    fn handle(&mut self, msg: server::SrvMessage, ctx: &mut Self::Context) -> usize {
        ctx.text(msg.msg);
        1
    }
}
