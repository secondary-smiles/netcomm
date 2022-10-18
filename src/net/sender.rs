use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use info_utils::error;
use info_utils::prelude::*;

use crate::util::{
    comms::Net,
    connection::Connection,
    message::Message,
};

pub fn create_sender_connection(connection: Connection, comms: Net) {
    let stream = TcpStream::connect(
        format!("{}:{}",
                connection.domain.should("Should be validated"),
                connection.port.should("Should be validated")
        )).eval_or_else(|e| {
        error!("{}", e);
    });

    let mut l_stream = stream;
    let mut s_stream = l_stream.try_clone().should("System error");

    let (sig, recvr) = mpsc::channel();

    let listen_thread = thread::Builder::new()
        .name("Net Listener".to_string())
        .spawn(move || {
            loop {
                let mut buffer = [0; 8192];
                let bytes_read = l_stream.read(&mut buffer).eval_or_default();
                l_stream.flush().should("Stream should flush successfully");
                if bytes_read == 0 {
                    if l_stream.peek(&mut buffer).eval_or_default() == 0 {
                        warn!("stream closed");
                        sig.send(true).should("Channel error");
                        break;
                    }
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


    let send_thread = thread::Builder::new()
        .name("Net Sender".to_string())
        .spawn(move || {
            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();
                for message in incoming_messages {
                    s_stream.write(message.content.as_bytes()).eval_or_default();
                    s_stream.flush().should("Stream should flush successfully");
                }
                let status = recvr.try_recv().eval_or_default();
                if status {
                    break;
                }
            }
        }).eval();

    listen_thread.join().should("Thread should panic");
    send_thread.join().should("Thread should panic");
}