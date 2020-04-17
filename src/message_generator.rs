use tokio::sync::mpsc::{Sender,Receiver};
use tokio::sync::oneshot;

use crate::time::{sleep};

mod message;
pub use message::{Message};


/// Control Request Type
#[derive(Debug)]
pub enum Ctrl {
    Quit,
    Health
}

/// Control Repsonse Type
#[derive(Debug)]
pub enum CtrlR {
    Quit(QuitR),
    Health(HealthR),
}

#[derive(Debug)]
pub enum QuitR {
   Ok
}

#[derive(Debug)]
pub enum HealthR {
    Healthy,
    UnHealthy
}

/// The spawn function of the agent
pub async fn message_generator(mut ctrl: Receiver<(Ctrl, oneshot::Sender<CtrlR>)>, mut channel: Sender<Message>) {
    loop {
        tokio::select! {
            msg = channel.send(Message::Hello) =>
                match msg  {
                    Ok(()) => sleep(500).await,
                    Err(_) => {
                        eprintln!("Error sending message");
                        break;
                    }
                },
            ctl = ctrl.recv() => {
                match ctl {
                    Some((Ctrl::Quit, rtx)) => {
                        rtx.send(CtrlR::Quit(QuitR::Ok)).expect("unable to respond to ctrl message");
                        break;
                    },
                    Some((Ctrl::Health, rtx)) => {
                        rtx.send(CtrlR::Health(HealthR::Healthy)).expect("unable to respond to ctrl message");
                    }
                    None => break // all senders have dropped
                }
            }
        }
    }
    println!("Message generator stopped");
}
