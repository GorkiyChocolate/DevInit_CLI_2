mod api;
mod structs;
mod yaml_config;
mod cli;
mod commands;

#[tokio::main]
async fn main(){
    let client: reqwest::Client = reqwest::Client::new();
    let method = "GET";
    let url = "https://api.example.com/data";
    let body = Some("{\"key\": \"value\"}");

    cli::cli_logic().await;
    api::get_recipe("localhost:3000", "redis" ).await;
    yaml_config::append_data();
}   