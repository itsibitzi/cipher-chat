use clap::{Parser, Subcommand};
use reqwest::Url;

#[derive(Parser)]
pub struct Cli {
    #[clap(long)]
    pub api_url: Url,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Register {},
    AddFriend {},
    CreateGroup {},
    ViewGroupMessages {},
    AddToGroup {},
    SendMessageToGroup {},
    UpdateStatus {},
}
