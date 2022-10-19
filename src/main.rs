use clap::Parser;
use info_utils::prelude::*;
use std::sync::mpsc;
use std::thread;

mod util;
mod net;
mod repl;

use util::{
    comms::{Net, Repl},
    message::Message,
    connection::Connection,
};
use crate::net::listener::create_listener;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
/// Communicate with servers interactively
pub struct Args {
    domain: String,

    port: u16,

    /// Run in listen mode
    #[arg(short, long)]
    listen: bool,
}

fn main() {
    let mut args = Args::parse();
    args.validate().eval_or_else(|e| {
        error!("Invalid program arguments: {}", e);
    });

    let (n_sender, n_reciever) = mpsc::channel::<Message>();
    let (r_sender, r_reciever) = mpsc::channel::<Message>();
    let (ne_sender, ne_reciever) = mpsc::channel::<bool>();
    let (re_sender, re_reciever) = mpsc::channel::<bool>();

    let net_comms = Net {
        sender: n_sender,
        recvr: r_reciever,
        event_o: re_sender,
        event_i: ne_reciever,
    };

    let repl_comms = Repl {
        sender: r_sender,
        recvr: n_reciever,
        event_o: ne_sender,
        event_i: re_reciever,
    };

    let connection = Connection {
        domain: args.domain.clone(),
        port: args.port,
    };


    if args.listen {
        thread::Builder::new()
            .name("Net".to_string())
            .spawn(move || {
                create_listener(connection, net_comms);
            }).eval();
    } else {
        thread::Builder::new()
            .name("Net".to_string())
            .spawn(move || {
                net::sender::create_sender_connection(connection, net_comms);
            }).eval();
    };
    repl::print::create_repl(repl_comms);
}
