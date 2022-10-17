use std::sync::mpsc;

use crate::message::Message;

pub struct Net {
    // Messages from external source to repl to be displayed; net -> repl
    pub inbound_sender: mpsc::Sender<Message>,
    pub inbound_recvr: mpsc::Receiver<Message>,
}

pub struct Repl {
    // Messages from the repl to be sent; repl -> net
    pub outbound_sender: mpsc::Sender<Message>,
    pub outbound_recvr: mpsc::Receiver<Message>,
}