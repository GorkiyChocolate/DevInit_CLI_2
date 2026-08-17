mod api;
mod cli;
mod commands;
mod structs;
mod yaml_config;

#[tokio::main]
async fn main() {
    cli::cli_logic().await;
}