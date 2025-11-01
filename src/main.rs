pub mod colorscheme;
pub mod theme_listener;
pub mod config;
pub mod theme;

#[tokio::main]
async fn main() {
    theme_listener::listen_theme_changes().await.unwrap();
}
