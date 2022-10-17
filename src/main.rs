use clap::Parser;
use info_utils::prelude::*;

mod validate;
mod log;
mod message;
mod comms;

use crate::log::LogUtil;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
/// Communicate with servers interactively
pub struct Args {
    port: u16,

    domain: Option<String>,

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
}
