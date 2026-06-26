use std::sync::mpsc;
use crate::message::IpcMessage;

pub struct IpcSender(mpsc::Sender<IpcMessage>);
pub struct IpcReceiver(mpsc::Receiver<IpcMessage>);

pub fn ipc_channel() -> (IpcSender, IpcReceiver) {
    let (tx, rx) = mpsc::channel();
    (IpcSender(tx), IpcReceiver(rx))
}

impl IpcSender {
    pub fn send(&self, msg: IpcMessage) -> Result<(), mpsc::SendError<IpcMessage>> {
        self.0.send(msg)
    }
}

impl IpcReceiver {
    pub fn try_recv(&self) -> Result<IpcMessage, mpsc::TryRecvError> {
        self.0.try_recv()
    }
    pub fn recv(&self) -> Result<IpcMessage, mpsc::RecvError> {
        self.0.recv()
    }
}
