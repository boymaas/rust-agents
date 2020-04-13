#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use tokio::sync::mpsc::{channel,Sender,Receiver};
use tokio::sync::oneshot;
use tokio::time::{Delay,delay_for};
use tokio::prelude::*;

use failure::{Fallible};

mod time;
use time::{sleep};

mod message_generator;
use message_generator::{message_generator, Message, Ctrl};

mod file_sink;
use file_sink::{file_sink};

#[tokio::main]
async fn main() -> Fallible<()> {
    let (tx,rx) = channel::<Message>(10);

    let (mut ctx,crx) = channel::<Ctrl>(10);

    // message_generation -> file_sink
    tokio::spawn(message_generator(crx, tx));
    tokio::spawn(file_sink(rx));

    sleep(2000).await; // print 20 messages


    println!("Health message send ..");

    let (rtx,rrx) = oneshot::channel();
    ctx.send(Ctrl::Health(rtx)).await?;
    let response = rrx.await?;
    println!("Received health resp {:?}!", response);

    sleep(2000).await; // print 20 messages

    println!("Quit message send ..");
    ctx.send(Ctrl::Quit).await?;

    sleep(2000).await; // print 20 messages

    println!("Exiting program");

    Ok(())
}
