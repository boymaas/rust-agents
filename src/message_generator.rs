use tokio::sync::mpsc::{channel,Sender,Receiver};
use tokio::sync::oneshot;
use tokio::select;

use crate::time::{sleep};

mod message;
pub use message::{Message};


// Ctrl channel
// Quit
// Health -> Healty or not
#[derive(Debug)]
pub enum Ctrl {
    Quit,
    Health(oneshot::Sender<HealthResponse>)
}

#[derive(Debug)]
pub enum HealthResponse {
    Healthy,
    UnHealthy
}

pub async fn message_generator(mut ctrl: Receiver<Ctrl>, mut channel: Sender<Message>) {
    loop {
        tokio::select! {
            msg = channel.send(Message::Hello) =>
                match msg  {
                    Ok(()) => sleep(100).await,
                    Err(_) => {
                        eprintln!("Error sending message");
                        break;
                    }
                },
            ctl = ctrl.recv() => {
                match ctl {
                    Some(Ctrl::Quit) => break,
                    Some(Ctrl::Health(rtx)) => {
                        rtx.send(HealthResponse::Healthy).unwrap()
                    }
                    None => break // all senders have dropped
                } 
            }
        }
    }
    println!("Message generator stopped");
}
