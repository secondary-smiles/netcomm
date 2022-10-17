use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use info_utils::error;
use info_utils::prelude::*;

use crate::util::{
    comms::Net,
    connection::Connection,
    message::Message,
};

pub fn create_sender_connection(connection: Connection, comms: Net) {
    let mut stream = Arc::new(
        Mutex::new(
            TcpStream::connect(
                format!("{}:{}",
                        connection.domain.should("Should be validated"),
                        connection.port.should("Should be validated")
                )
            ).eval_or_else(|e| {
                error!("{}", e);
            })));

    let l_stream = Arc::clone(&mut stream);
    let s_stream = Arc::clone(&mut stream);

    let listen_thread = thread::Builder::new()
        .name("Listener Thread".to_string())
        .spawn(move || {
            let mut l_stream = l_stream.lock().should("Mutex is poisoned");
            loop {
                let mut buffer = [0; 8192];
                let bytes_read = l_stream.read(&mut buffer).eval_or_default();
                if bytes_read == 0 {
                    if l_stream.peek(&mut buffer).eval_or_default() == 0 {
                        warn!("stream closed");
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
        .name("Sender Thread".to_string())
        .spawn(move || {
            let mut s_stream = s_stream.lock().should("Mutex is poisoned");

            loop {
                let incoming_messages: Vec<Message> = comms.recvr.try_iter().collect();
                let mut bytes_sent: usize = 0;

                for message in incoming_messages {
                    bytes_sent = s_stream.write(message.content.as_bytes()).eval_or_default();
                }

                if bytes_sent == 0 {
                    let mut buffer = [0; 1];
                    if s_stream.peek(&mut buffer).eval_or_default() == 0 {
                        warn!("stream closed");
                        break;
                    }
                }
            }
        }).eval();

    listen_thread.join().should("Thread should panic");
    send_thread.join().should("Thread should panic");
}