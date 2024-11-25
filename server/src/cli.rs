use std::{net::SocketAddrV4, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub db_path: PathBuf,
    pub listen_on: SocketAddrV4,
}
