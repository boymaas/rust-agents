#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use failure::{Fallible};

mod time;
use time::{sleep};

mod message_generator;
use crate::message_generator::{Ctrl};

mod file_sink;

mod message_recorder;
use message_recorder::{MessageRecorder};

mod message_subject_scanner;
use message_subject_scanner::{MessageSubjectScanner};

mod message_recorder_spawner;
use message_recorder_spawner::{MessageRecorderSpawner};

// MessageSubjectScanner <= Episode seven
// will poll on an interval to get the current list of subjects.

// MessageRecorderSpawner <= Episode eight
// listens to the MessageScanner for updates in the list of messages
// and will update a local HashMap of subjects with handle the spawned
// recorders. Will remove the recorders not active anymore, and will add
// recorders not recording yet.

// MessageRecorder <= Episode 10
// a system that spawns a message_generator, and file_sink. Based on the
// name of the messages.

#[tokio::main]
async fn main() -> Fallible<()> {
    let msgss = MessageSubjectScanner::spawn()?;
    let msgr = MessageRecorderSpawner::spawn(msgss)?;

    sleep(20000).await; // print 20 messages

    println!("Exiting program");

    Ok(())
}
