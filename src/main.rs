mod api;
mod cli;
mod file_config;
mod models;
use cli::cli_logic;

#[tokio::main]
async fn main() {
    cli_logic::cli_logic().await;
}