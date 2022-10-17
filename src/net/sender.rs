use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use info_utils::error;
use info_utils::prelude::*;

use crate::util::{
    comms::Net,
    connection::Connection,
};

pub fn create_sender_connection(connection: Connection, comms: Net) {
    let mut stream = TcpStream::connect(format!("{}:{}", connection.domain.should("Should be validated"), connection.port.should("Should be validated"))).eval_or_else(|e| {
        error!("{}", e);
    });

    let listen_thread = thread::Builder::new()
        .name("Listener Thread".to_string())
        .spawn(|| {
            terror!("test error");
        }).eval();


    let send_thread = thread::Builder::new()
        .name("Sender Thread".to_string())
        .spawn(|| {
            terror!("test error");
        }).eval();

    listen_thread.join().should("Thread should panic");
    send_thread.join().should("Thread should panic");
}