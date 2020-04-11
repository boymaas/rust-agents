use tokio::sync::mpsc::{channel,Sender,Receiver};

use crate::time::{sleep};

mod message;
pub use message::{Message};

pub async fn message_generator(mut channel: Sender<Message>) {
    loop {
        match channel.send(Message::Hello).await {
            Ok(()) => sleep(100).await,
            Err(_) => {
                eprintln!("Error sending message");
                break;
            }
        }
    }
}
