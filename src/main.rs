use clap::Parser;

pub mod args;
pub mod colorscheme;
pub mod config;
pub mod theme;
pub mod theme_listener;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    args.command().await;
}
