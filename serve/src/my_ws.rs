use super::server;
use actix::prelude::*;
use actix_web_actors::ws;

type AddrServer = Addr<server::Server>;

#[derive(Debug)]
pub struct MyWs {
    pub addr: AddrServer,
    pub id: usize,
}

impl MyWs {
    pub fn new(addr_: AddrServer) -> Self {
        Self { addr: addr_, id: 0 }
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is alive");
        let res = self
            .addr
            .send(server::Connect {
                ctx: _ctx.address(),
            })
            .into_actor(self)
            .then(|res, _ws, _ctx| {
                match res {
                    Ok(id) => println!("{id}"),
                    _ => println!("err"),
                }
                fut::ready(())
            })
            .wait(_ctx);
        dbg!(res);
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
                self.addr
                    .send(server::TestMsg {
                        msg: text.to_string(),
                    })
                    .into_actor(self)
                    .then(|res, _act, ctx| {
                        match res {
                            Ok(res) => println!("{res}"),
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

impl Handler<server::TestMsg> for MyWs {
    type Result = usize;

    fn handle(&mut self, msg: server::TestMsg, ctx: &mut Self::Context) -> usize {
        ctx.text(msg.msg);
        1
    }
}
