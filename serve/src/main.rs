use actix::prelude::*;
use actix_files as fs;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

struct MyWs;
impl MyWs {
    fn new() -> Self {
        Self
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

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("into handle");
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("message : {text}");
                ctx.text(format!("hello from rs server: {text}"))
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
async fn ws_r(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("ws");
    let resp = ws::start(MyWs::new(), &req, stream);
    resp
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("good jobs")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("hello word");
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(ws_r)
            .service(fs::Files::new("/test", "static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
