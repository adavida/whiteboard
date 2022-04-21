use actix::prelude::*;
use actix_files as fs;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::env;

mod server;

#[derive(Debug)]
pub struct MyWs {
    pub addr: Addr<server::Server>,
    pub id: usize,
}

impl MyWs {
    fn new(addr_: Addr<server::Server>) -> Self {
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

#[get("/ws")]
async fn ws_r(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs::new(srv.get_ref().clone()), &req, stream);
    resp
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("good jobs")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    let server = server::Server::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .service(root)
            .service(ws_r)
            .service(fs::Files::new("/test", "static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
