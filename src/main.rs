use std::time::Duration;

use actix::prelude::*;

use actix_rt::time;
use consumer::{Consumer, Get};
use producer::Producer;

mod consumer;
mod producer;

#[actix_rt::main]
async fn main() {
    let consumer = Consumer::new().start();
    Producer::new(consumer.clone().recipient()).start();

    time::sleep(Duration::from_secs(1)).await;

    let count = consumer.send(Get).await.unwrap();
    println!("Counted {}", count);

    System::current().stop();
}
