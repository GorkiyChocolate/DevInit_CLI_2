mod commands;
mod api;
mod structs;
mod append_data;
mod cli;

#[tokio::main]
async fn main(){
    cli::cli_logic();
}