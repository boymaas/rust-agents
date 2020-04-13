use tokio::sync::mpsc::{channel,Sender,Receiver};

pub async fn file_sink<T: core::fmt::Debug>(mut channel: Receiver<T>) {
    while let Some(msg) = channel.recv().await {
        println!("Writing to file {:?}", msg);
    }
}
