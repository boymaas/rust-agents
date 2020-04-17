use tokio::fs::File;
use tokio::prelude::*;

use std::fmt::Debug;

use failure::{Fallible};
use serde::{Serialize};

use tokio::sync::mpsc::{Receiver};

pub async fn file_sink<T: Debug + Serialize>(filepath: &'static str, mut channel: Receiver<T>) -> Fallible<()> {
    // open a file
    let mut file = File::create(format!("data/{}.data.bin", filepath)).await
        .expect("cannot open file");

    while let Some(msg) = channel.recv().await {
        println!("Writing to file {} msg {:?}", filepath, msg);
        // write the variable in binary format
        // to a file
        file.write(&bincode::serialize(&msg)?).await
            .expect("cannot write to file");
    }

    // close the file on exit
    Ok(())
}
