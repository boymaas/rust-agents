#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


use tokio::sync::mpsc::{channel,Sender,Receiver};
use tokio::time::{Delay,delay_for};
use tokio::prelude::*;

mod time;
use time::{sleep};

mod message_generator;
use message_generator::{message_generator, Message};

mod file_sink;
use file_sink::{file_sink};

#[tokio::main]
async fn main() {
    let (tx,rx) = channel::<Message>(10);

    // message_generation -> file_sink
    tokio::spawn(message_generator(tx));
    tokio::spawn(file_sink(rx));

    sleep(2000).await;
}
