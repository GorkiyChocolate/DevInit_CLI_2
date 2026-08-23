mod api {
    pub mod add_recipe;
    pub mod get_config;
}
mod cli {
    pub mod cli_logic;
    pub mod commands;
}
mod file_config {
    pub mod env_config;
    pub mod yaml_config;
}
mod models {
    pub mod structs;
}

use cli::cli_logic::cli_logic;

#[tokio::main]
async fn main() {
    cli_logic().await;
}
