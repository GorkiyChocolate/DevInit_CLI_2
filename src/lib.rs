pub mod api {
    pub mod add_recipe;
    pub mod get_config;
}
pub mod cli {
    pub mod cli_logic;
    pub mod commands;
}
pub mod file_config {
    pub mod env_config;
    pub mod yaml_config;
}
pub mod models {
    pub mod structs;
}

pub use cli::cli_logic::cli_logic;
