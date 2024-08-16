use std::{net::SocketAddr, str::FromStr};

use clap::Parser;
use cli::Args;
use log::info;
use server::QuickServer;

mod server;
mod cli;

const VERSION: &str = "0.1.0";

#[tokio::main]
async fn main() {
    let args = Args::parse();

    env_logger::init();
    info!("Starting QuickServer v{}...", VERSION);
    
    let host = SocketAddr::from_str(&format!("{}:{}", args.host, args.port)).unwrap_or_else(|_| panic!("Invalid socket address: {}:{}", args.host, args.port));
    let root = args.root;
    let index = args.index;

    let server = QuickServer::new(host, root, index);
    server.serve().await;
}
