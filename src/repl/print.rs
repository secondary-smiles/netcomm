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

pub fn create_repl(comms: Repl) -> () {
    let handle_thread = thread::Builder::new()
        .name("UI Listener".to_string())
        .spawn(move || {
            let mut stdin = termion::async_stdin().keys();
            let mut stdout = stdout().into_raw_mode().eval();

            let mut current_line = String::new();

            write!(stdout, "> ").eval();
            loop {
                let status = comms.event_i.try_recv().eval_or_default();
                if status {
                    shutdown(comms);
                    break;
                }

                let input = stdin.next();
                if let Some(Ok(key)) = input {
                    match key {
                        Key::Char('\n') => {
                            current_line += "\r\n";
                            comms.sender.send(Message::new("YOU".to_string(), current_line)).eval();
                            current_line = String::new();
                            write!(stdout, "{}\r> ", termion::clear::CurrentLine).eval();
                            stdout.flush().eval();
                        }
                        Key::Char(c) => {
                            write!(stdout, "{}", c as char).eval();
                            stdout.flush().eval();

                            current_line.push(c as char);
                        }
                        Key::Ctrl('c') => {
                            shutdown(comms);
                            break;
                        }
                        _ => {}
                    }
                }

                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();

                for message in incoming_messages {
                    println!("{}", message)
                }

                fn shutdown(comms: Repl) {
                    comms.event_o.send(true).eval();
                    println!("Restoring terminal, please hold..\r");
                }
            }
        }).eval();
    handle_thread.join().eval();
    return;
}