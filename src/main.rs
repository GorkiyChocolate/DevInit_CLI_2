mod api;
mod structs;
mod append_data;
mod cli;
mod commands;

#[tokio::main]
async fn main(){
    let client: reqwest::Client = reqwest::Client::new();
    let method = "GET";
    let url = "https://api.example.com/data";
    let body = Some("{\"key\": \"value\"}");

    cli::cli_logic().await;
    api::execute_request(client, method, url, body).await;
    append_data::append_data()
}   