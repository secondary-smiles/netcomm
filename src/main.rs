use clap::Parser;
use info_utils::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

mod util;
mod net;
mod repl;

use util::{
    log::LogUtil,
    comms::{Net, Repl},
    message::Message,
    connection::Connection,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
/// Communicate with servers interactively
pub struct Args {
    domain: Option<String>,

    port: Option<u16>,

    /// Run in listen mode
    #[arg(short, long)]
    listen: bool,

    /// Silence output logs
    #[arg(short, long)]
    quiet: bool,

    /// Print extra logs
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let mut args = Args::parse();
    args.validate().eval_or_else(|e| {
        error!("Invalid program arguments: {}", e);
    });

    let (n_sender, n_reciever) = mpsc::channel::<Message>();
    let (r_sender, r_reciever) = mpsc::channel::<Message>();
    let (e_sender, e_reciever) = mpsc::channel::<bool>();

    let net_comms = Net {
        sender: n_sender,
        recvr: r_reciever,
        event: e_sender,
    };

    let repl_comms = Repl {
        sender: r_sender,
        recvr: n_reciever,
        event: e_reciever,
    };

    let connection = Connection {
        domain: args.domain.clone(),
        port: args.port.clone(),
    };


    let net_thread: JoinHandle<()>;
    if args.listen {
        args.log_v("Creating listener server..");
        net_thread = thread::Builder::new()
            .name("Net".to_string())
            .spawn(move || {
                net::sender::create_sender_connection(connection, net_comms);
            }).eval();
    } else {
        args.log_v("Creating sender connection..");
        net_thread = thread::Builder::new()
            .name("Net".to_string())
            .spawn(move || {
                net::sender::create_sender_connection(connection, net_comms);
            }).eval();
    }
    repl::print::create_repl(repl_comms);
    net_thread.join().eval();
}
