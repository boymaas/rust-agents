use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;

use failure::{Fallible};
use serde::{Serialize, Deserialize};

use tokio::sync::mpsc::{channel,Sender,Receiver};

pub async fn file_sink<T: Debug + Serialize>(mut channel: Receiver<T>) -> Fallible<()> {
    // open a file
    let mut file = File::create("data.bin")?;

    while let Some(msg) = channel.recv().await {
        println!("Writing to file {:?}", msg);
        // write the variable in binary format
        // to a file
        file.write(&bincode::serialize(&msg)?)?;
    }

    // close the file on exit
    Ok(())
}
