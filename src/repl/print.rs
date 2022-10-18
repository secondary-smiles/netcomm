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

            let mut current_line = String::new();

            write!(stdout, "> ").eval();
            loop {
                let input = stdin.next();
                if let Some(Ok(key)) = input {
                    match key {
                        Key::Char(c) => {current_line.push(c)}
                        Key::Char('\n') => {}
                        Key::Ctrl('c') => {shutdown(); break;},
                        _ => {}
                    }
                }


                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

                for message in incoming_messages {
                    println!("{}", message)
                }

                let msg = Message {
                    sender: "EXT".to_string(),
                    content: "test message\r\n".to_string(),
                };


                let status = comms.event.try_recv().eval_or_default();
                if status {
                    shutdown();
                    break;
                }

                comms.sender.send(msg).eval();

                fn shutdown() {
                    println!("Restoring terminal, please hold..\r");
                }
            }
        }).eval();
    handle_thread.join().eval();
}