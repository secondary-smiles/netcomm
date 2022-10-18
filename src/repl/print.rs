use std::thread;
use std::time::Duration;
use info_utils::prelude::*;
use crate::util::{
    comms::Repl,
    message::Message,
};

pub fn create_repl(comms: Repl) {
    thread::Builder::new()
        .name("UI Listener".to_string())
        .spawn(move || {
            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

                for message in incoming_messages {
                    println!("{}", message)
                }

                let msg = Message {
                    sender: "EXT".to_string(),
                    content: "test message\r\n".to_string(),
                };

                comms.sender.send(msg).eval();
                thread::sleep(Duration::from_millis(1000));
            }
        }).eval();
}