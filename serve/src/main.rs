use actix::prelude::*;
use actix_files as fs;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::env;

mod my_ws;
mod server;

#[get("/ws")]
async fn ws_r(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::Server>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(my_ws::MyWs::new(srv.get_ref().clone()), &req, stream);
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
