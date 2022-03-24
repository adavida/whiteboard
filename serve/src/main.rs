use actix::prelude::*;
use actix_files as fs;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::env;

mod server;

struct MyWs {
    pub addr: Addr<server::Server>,
}

impl MyWs {
    fn new(addr: Addr<server::Server>) -> Self {
        Self { addr: addr }
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl Drop for MyWs {
    fn drop(&mut self) {
        println!("destoy ws");
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("into handle");
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("message : {text}");
                ctx.text(format!("hello from rs server: {text}"));
                self.addr
                    .send(server::TestMsg {
                        msg: text.to_string(),
                    })
                    .into_actor(self)
                    .then(|res, _act, ctx| {
                        match res {
                            Ok(res) => println!("== {res}"),
                            // something is wrong with chat server
                            _ => ctx.stop(),
                        }
                        fut::ready(())
                    })
                    .wait(ctx);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            // println!("handle close ");
            // dbg!(&reason);
            _ => (),
        }
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
