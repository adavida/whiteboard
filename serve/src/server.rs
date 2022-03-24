use actix::prelude::*;

pub struct Server;

#[derive(Message)]
#[rtype(usize)]
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
    type Result = usize;

    fn handle(&mut self, msg: TestMsg, _ctx: &mut Context<Self>) -> usize {
        println!("receive message {}", msg.msg);

        1
    }
}
