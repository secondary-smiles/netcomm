use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use info_utils::prelude::*;
use crate::Connection;

use crate::util::{
    comms::Net,
    message::Message,
};

pub fn create_listener(connection: Connection, comms: Net) {
    let listener = TcpListener::bind(
        format!("{}:{}",
                connection.domain,
                connection.port,
        )).eval_or_else(|e| {
        comms.event_o.send(true).should("Channel error");
        terror!("{e}");
    });

    for stream in listener.incoming() {
        let stream = stream.eval();
        handle_stream(stream, comms);
        break;
    }
}

fn handle_stream(stream: TcpStream, comms: Net) {
    let mut l_stream = stream;
    let mut s_stream = l_stream.try_clone().should("System error");

    let listen_thread = thread::Builder::new()
        .name("Net Listener".to_string())
        .spawn(move || {
            loop {
                let status = comms.event_i.try_recv().eval_or_default();
                if status {
                    comms.event_o.send(true).should("Channel error");
                    break;
                }

                let mut buffer = [0; 8192];
                let bytes_read = l_stream.read(&mut buffer).eval_or_default();
                l_stream.flush().should("Stream should flush successfully");
                if bytes_read == 0 && l_stream.peek(&mut buffer).eval_or_default() == 0 {
                    comms.event_o.send(true).should("Channel error");
                    break;
                }
                let read_message = String::from_utf8_lossy(&buffer);
                let read_message = read_message.trim_end_matches(char::from(0));

                let message = Message {
                    sender: "EXT".to_string(),
                    content: read_message.to_string(),
                };
                comms.sender.send(message).should("Sender should not be blocked");
            }
        }).eval();


    thread::Builder::new()
        .name("Net Sender".to_string())
        .spawn(move || {
            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();
                for message in incoming_messages {
                    s_stream.write(message.content.as_bytes()).eval_or_default();
                    s_stream.flush().should("Stream should flush successfully");
                }
            }
        }).eval();


    listen_thread.join().eval();
}