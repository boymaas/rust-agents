use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use tokio::sync::mpsc::{channel, Sender, Receiver};

use failure::Fallible;

use crate::message_recorder;
use crate::message_recorder::{MessageRecorder};
use crate::message_subject_scanner::{MessageSubjectScanner, Subjects};

pub struct MessageRecorderSpawner {
   ctrl_tx: Sender<Ctrl> 
}

#[derive(Debug)]
pub enum Ctrl {
   Quit 
}

impl MessageRecorderSpawner {
    pub fn spawn(msgss: MessageSubjectScanner) -> Fallible<MessageRecorderSpawner> {
        let (ctx, crx) = channel::<Ctrl>(10);

        tokio::spawn(agent_loop(crx, msgss));

        Ok(MessageRecorderSpawner {
           ctrl_tx: ctx 
        })
   }

    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<()> {
        self.ctrl_tx.send(msg).await;

        Ok(())
    }
}

pub async fn agent_loop(mut ctrl: Receiver<Ctrl>, mut msgss: MessageSubjectScanner) {
    // state
    let mut active_subjects: MsgRecRegistry = HashMap::new();

    loop {
        tokio::select! {
            msg = msgss.subjects_rx.recv() => {
                let subjects = msg.unwrap();
                manage_message_recorders(&mut active_subjects, &subjects).await;
            }
            ctl = ctrl.recv() => {
                match ctl {
                    Some(Ctrl::Quit) => break,
                    None => break
                }
            }
        }
    }
    println!("MessageRecorderSpawner finished");
}

type MsgRecRegistry = HashMap<&'static str,MessageRecorder>;

async fn manage_message_recorders(mrr: &mut MsgRecRegistry, subjects: &Subjects) {
    // currently active set
    let active: HashSet<&str> = HashSet::from_iter(mrr.keys().copied());
    let desired: HashSet<&str> = HashSet::from_iter(subjects.iter().copied());

    // desired active set
    // intersection(current, desired) => in common
    let intersection = active.intersection(&desired).copied().collect();

    // to_spawn = diff intersection, desired
    let to_spawn = desired.difference(&intersection);
    // to_quit  = diff intersection, active
    let to_quit = active.difference(&intersection);

    println!("to_quit {:?}", to_quit);
    for s in to_quit {
        println!("Quitting {}", s);
        let mut msgr = mrr.remove(s).unwrap();
        msgr.send_ctrl_msg(message_recorder::Ctrl::Quit).await.expect("cannot send quit message");
    }

    println!("to_spawn {:?}", to_spawn);
    for s in to_spawn {
        println!("Spawning {}", s);
        mrr.insert(s, MessageRecorder::spawn(s).expect("unable to spawn message recorder"));
    }
}
