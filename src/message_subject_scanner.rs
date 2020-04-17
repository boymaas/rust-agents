use tokio::sync::mpsc::{channel,Sender,Receiver};
use tokio::sync::oneshot;
use rand::prelude::*;

use failure::{Fallible};

use crate::time::{sleep};

const VALID_SUBJECTS: &'static [&'static str] = &["orange", "apples", "plumes", "apricot", "mandarin", "pear"];

// We only need Ctrl::Quit no health
// needed here, as we are just polling.
#[derive(Debug)]
pub enum Ctrl {
    Quit
}

pub struct MessageSubjectScanner {
    ctrl: Sender<Ctrl>,
    subjects: Vec<String>,
    pub subjects_rx: Receiver<Subjects>

}

impl MessageSubjectScanner  {
    pub fn spawn() -> Fallible<MessageSubjectScanner> {
        let (tx,rx) = channel::<Subjects>(10);

        let (ctx,crx) = channel::<Ctrl>(10);

        tokio::spawn(agent_loop(crx, tx));

        Ok(MessageSubjectScanner {
            subjects: Vec::new(),
            ctrl: ctx,
            subjects_rx: rx
        })
    }

    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<()> {
        self.ctrl.send(msg).await?; 

        Ok(())
    }
}

pub type Subjects = Vec<&'static str>;

fn build_random_subjects() -> Subjects {
    let mut subjects : Vec<&'static str> = VALID_SUBJECTS.into();

    let rng = &mut thread_rng();
    subjects.shuffle(rng);

    subjects.into_iter().take(rng.gen_range(1, VALID_SUBJECTS.len())).collect()
}

// The agent loop
pub async fn agent_loop(mut ctrl: Receiver<Ctrl>, mut channel: Sender<Subjects>) {
    loop {
        let subjects = build_random_subjects();

        tokio::select! {
            // TODO: example subject being send, change into randoms
            msg = channel.send(subjects) =>
                match msg  {
                    Ok(()) => sleep(2000).await,
                    Err(_) => {
                        eprintln!("Error sending message");
                        break;
                    }
                },
            // Very simple, just a quit. 
            ctl = ctrl.recv() => {
                match ctl {
                    Some(Ctrl::Quit) => {
                        break;
                    },
                    None => break // all senders have dropped
                }
            }
        }
    }
    println!("Message generator stopped");
}
