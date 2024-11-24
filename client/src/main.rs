use clap::Parser as _;
use cli::{Cli, Command};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Register {} => todo!(),
        Command::AddFriend {} => todo!(),
        Command::CreateGroup {} => todo!(),
        Command::ViewGroupMessages {} => todo!(),
        Command::AddToGroup {} => todo!(),
        Command::SendMessageToGroup {} => todo!(),
        Command::UpdateStatus {} => todo!(),
    }
}
