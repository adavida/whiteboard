use actix::prelude::*;
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
pub struct TestMsg {
    pub msg: String,
}

impl Server {
    pub fn new() -> Self {
        Server {
            addrs: HashMap::new(),
            count: 0,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<TestMsg> for Server {
    type Result = usize;

    fn handle(&mut self, msg: TestMsg, _ctx: &mut Context<Self>) -> usize {
        for (_id, _ctx) in self.addrs.iter() {
            _ctx.do_send(TestMsg {
                msg: msg.msg.to_string(),
            });
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
