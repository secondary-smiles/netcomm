use std::thread;
use std::time::Duration;
use info_utils::prelude::*;
use crate::util::{
    comms::Repl,
    message::Message,
};

pub fn create_repl(comms: Repl) {
    loop {
        let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

        for message in incoming_messages {
            log!("{}", message)
        }

        let msg = Message {
            sender: "EXT".to_string(),
            content: "test message".to_string()
        };
    }
}