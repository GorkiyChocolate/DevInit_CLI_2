pub mod api {
    pub mod add_recipe;
    pub mod get_config;
    pub mod login;
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
    pub mod docker_compose_struct;
    pub mod k8s_struct;
    pub mod cicd_struct;
}

pub mod validator{
    pub mod k8s_validator;
}

pub mod generator{
    pub mod k8s_generator;
    pub mod cicd_generator;
}

pub use cli::cli_logic::cli_logic;
