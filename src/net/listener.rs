use std::io::prelude::*;
use std::net::TcpListener;
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
}