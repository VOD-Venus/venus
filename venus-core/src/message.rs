use std::sync::mpsc::{Receiver, Sender};

pub type Message = (Sender<MessageType>, Receiver<MessageType>);

#[derive(Debug)]
pub enum MessageType {
    /// Message from v2ray core process
    Core(String),
}
