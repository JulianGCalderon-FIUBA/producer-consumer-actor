use actix::prelude::*;

pub struct Consumer {
    count: usize,
}

impl Consumer {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Actor for Consumer {
    type Context = Context<Self>;
}

pub struct Consume(pub usize);
impl Message for Consume {
    type Result = ();
}

impl Handler<Consume> for Consumer {
    type Result = ();

    fn handle(&mut self, msg: Consume, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
    }
}

pub struct Get;
impl Message for Get {
    type Result = usize;
}

impl Handler<Get> for Consumer {
    type Result = usize;

    fn handle(&mut self, _msg: Get, _ctx: &mut Self::Context) -> Self::Result {
        self.count
    }
}
