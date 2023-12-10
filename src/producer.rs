use std::time::Duration;

use actix::prelude::*;

use crate::consumer::Consume;

const SLEEP_MILLIS: u64 = 10;
const RANDOM_RANGE: usize = 25;

pub struct Producer {
    consumer: Recipient<Consume>,
}

impl Producer {
    pub fn new(consumer: Recipient<Consume>) -> Self {
        Self { consumer }
    }
}

impl Actor for Producer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.address().do_send(Produce);
    }
}

struct Produce;
impl Message for Produce {
    type Result = ();
}

impl Handler<Produce> for Producer {
    type Result = ();

    fn handle(&mut self, _msg: Produce, ctx: &mut Self::Context) -> Self::Result {
        self.consumer
            .do_send(Consume(rand::random::<usize>() % RANDOM_RANGE));

        ctx.notify_later(Produce, Duration::from_millis(SLEEP_MILLIS));
    }
}
