use std::sync::mpsc;

use crate::util::message::Message;

pub struct Net {
    // Messages from external source to repl to be displayed; net -> repl
    pub sender: mpsc::Sender<Message>,
    pub recvr: mpsc::Receiver<Message>,
    pub event_o: mpsc::Sender<bool>,
    pub event_i: mpsc::Receiver<bool>
}

pub struct Repl {
    // Messages from the repl to be sent; repl -> net
    pub sender: mpsc::Sender<Message>,
    pub recvr: mpsc::Receiver<Message>,
    pub event_o: mpsc::Sender<bool>,
    pub event_i: mpsc::Receiver<bool>
}