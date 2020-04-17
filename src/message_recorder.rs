use tokio::sync::mpsc::{channel,Sender};
use tokio::sync::oneshot;

use failure::{Fallible};

pub use crate::message_generator::{message_generator, Message, Ctrl, CtrlR};

use crate::file_sink::{file_sink};

pub struct MessageRecorder {
    ctrl: Sender<(Ctrl, oneshot::Sender<CtrlR>)> 
}

impl MessageRecorder {
    /// Spawns a MessageRecorder, will spawn
    /// two agents. A message generator and a file_sink
    /// connect the two, and will return a handle to
    /// the agents in the form of a control channel
    pub fn spawn(filepath: &'static str) -> Fallible<MessageRecorder> {
        let (tx,rx) = channel::<Message>(10);

        let (ctx,crx) = channel::<(Ctrl, oneshot::Sender<CtrlR>)>(10);

        // message_generation -> file_sink
        tokio::spawn(message_generator(crx, tx));
        tokio::spawn(file_sink(&filepath, rx));

        Ok(MessageRecorder { ctrl: ctx })
    }

    /// Sends a ctrl message to the spawned agents.
    pub async fn send_ctrl_msg(&mut self, msg: Ctrl) -> Fallible<CtrlR> {
        let (rtx,rrx) = oneshot::channel::<CtrlR>();
        self.ctrl.send((msg, rtx)).await?; 

        Ok(rrx.await?)
    }

}
