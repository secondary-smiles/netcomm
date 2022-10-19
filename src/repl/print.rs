use std::thread;
use info_utils::prelude::*;
use termion::{raw::{IntoRawMode}, input::TermRead, event::Key, clear};
use std::io::{Write, stdout, stderr};


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
            let mut stderr = stderr().into_raw_mode().eval();

            let mut current_line = String::new();

            write!(stderr, "> ").eval();
            stderr.flush().eval();
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
                            comms.sender.send(Message::new(&"YOU".to_string(), &current_line)).eval_or_else(|e| {
                                warn!("Stream send error: {e}");
                            });
                            write!(stdout, "{}\r{}", clear::CurrentLine, Message::new(&"YOU".to_string(), &current_line)).eval();
                            write!(stderr, "{}\r> ", clear::CurrentLine).eval();
                            stdout.flush().eval();
                            stderr.flush().eval();
                            current_line = String::new();
                        }
                        Key::Char(c) => {
                            write!(stdout, "{}", c as char).eval();
                            stdout.flush().eval();

                            current_line.push(c as char);
                        }
                        Key::Backspace => {
                            let mut chars = current_line.chars();
                            chars.next_back();
                            let audited_line = chars.as_str();
                            write!(stderr, "{}\r> ", clear::CurrentLine).eval();
                            write!(stdout, "{}", audited_line).eval();
                            stdout.flush().eval();
                            stderr.flush().eval();

                            current_line = audited_line.to_string();
                        }
                        Key::Ctrl('c') => {
                            shutdown(comms);
                            break;
                        }
                        _ => {}
                    }
                }

                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();
                if !incoming_messages.is_empty() {
                    write!(stdout, "{}\r", clear::CurrentLine).eval();
                    for mut message in incoming_messages {
                        message.content = message.content.trim().to_string();
                        write!(stdout, "{}\r\n", message).eval();
                    }
                    write!(stderr, "> ").eval();
                    stderr.flush().eval();
                }

                fn shutdown(comms: Repl) {
                    comms.event_o.send(true).eval_or_else(|e| {
                        warn!("Network closed: {e}");
                    });
                    eprint!("\r\n\x1b[1mRestoring terminal, please hold..\x1b[0m\r\n");
                }
            }
        }).eval();
    handle_thread.join().eval();
}