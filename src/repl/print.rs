use std::thread;
use info_utils::prelude::*;
use termion::{
    screen,
    raw::{IntoRawMode},
    input::TermRead,
    event::Key,
};
use std::io::{Write, stdout, stdin};


use crate::util::{
    comms::Repl,
    message::Message,
};

pub fn create_repl(comms: Repl) {
    let handle_thread = thread::Builder::new()
        .name("UI Listener".to_string())
        .spawn(move || {
            let mut stdin = termion::async_stdin().keys();
            let mut stdout = stdout().into_raw_mode().eval();

            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

                for message in incoming_messages {
                    println!("{}", message)
                }

                let msg = Message {
                    sender: "EXT".to_string(),
                    content: "test message\r\n".to_string(),
                };

                let mut should_break = false;
                comms.sender.send(msg).eval_or_else(|e| {
                    should_break = true;
                    warn!("{e}");
                });
                if should_break { break; };
            }
        }).eval();
    handle_thread.join().eval();
}