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

    let (n_sender, n_receiver) = mpsc::channel::<Message>();
    let (r_sender, r_receiver) = mpsc::channel::<Message>();
    let (ne_sender, ne_receiver) = mpsc::channel::<bool>();
    let (re_sender, re_receiver) = mpsc::channel::<bool>();

    let net_comms = Net {
        sender: n_sender,
        recvr: r_receiver,
        event_o: re_sender,
        event_i: ne_receiver,
    };

    let repl_comms = Repl {
        sender: r_sender,
        recvr: n_receiver,
        event_o: ne_sender,
        event_i: re_receiver,
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
