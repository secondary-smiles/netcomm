use std::thread;
use std::time::Duration;
use info_utils::prelude::*;
use crate::util::{
    comms::Repl,
    message::Message,
};

pub fn create_repl(comms: Repl) {
    let listen_thread = thread::Builder::new()
        .name("UI Listener".to_string())
        .spawn(move || {
            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

                for message in incoming_messages {
                    println!("{}", message)
                }
            }
        }).eval();

    let send_thread = thread::Builder::new()
        .name("UI Sender".to_string())
        .spawn(move || {
            loop {
                let msg = Message {
                    sender: "EXT".to_string(),
                    content: "test message\r\n\r\n".to_string(),
                };

                comms.sender.send(msg).eval();
                thread::sleep(Duration::from_millis(1000));
            }
        }).eval();
    // listen_thread.join().eval();
    // send_thread.join().eval();
}