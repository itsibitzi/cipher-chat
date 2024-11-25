use axum::{routing::get, Router};
use clap::Parser as _;
use cli::Cli;

mod cli;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    let app = Router::new().route("/", get(|| async { "hello cipher-chat!" }));

    let listener = tokio::net::TcpListener::bind(cli.listen_on).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
