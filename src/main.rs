use clap::Parser;
use info_utils::prelude::*;
use std::sync::mpsc;
use std::thread;

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

    let net_comms = Net {
        sender: n_sender,
        recvr: r_reciever,
    };

    let repl_comms = Repl {
        sender: r_sender,
        recvr: n_reciever,
    };

    let connection = Connection {
        domain: args.domain.clone(),
        port: args.port.clone(),
    };

    let repl_thread = thread::Builder::new()
        .name("UI Thread".to_string())
        .spawn(move || {
            repl::print::create_repl(repl_comms);
        }).eval();

    if args.listen {
        args.log_v("Creating listener server..");
    } else {
        args.log_v("Creating sender connection..");
        net::sender::create_sender_connection(connection, net_comms);
    }
    repl_thread.join().eval();
}
