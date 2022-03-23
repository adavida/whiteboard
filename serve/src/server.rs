use actix::prelude::*;

pub struct Server;

#[derive(Message)]
#[rtype(result = "()")]
pub struct TestMsg {
    pub msg: String,
}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<TestMsg> for Server {
    type Result = ();

    fn handle(&mut self, msg: TestMsg, _ctx: &mut Context<Self>) {
        println!("receive message {}", msg.msg);
    }
}
