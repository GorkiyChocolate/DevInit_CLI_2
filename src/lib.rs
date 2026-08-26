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

pub mod ast_yml{
    pub mod generators{
        pub mod dockercompose{
            pub mod docker_generator;
            pub mod docker_compose_generator;
        }
        pub mod kubernetes {
            pub mod k8s_deployments;
        }
        pub mod cicd{
            pub mod github_actions;
            pub mod gitlab_ci;
        }
    }
    pub mod ast_structs;
    pub mod fswriteer;
    pub mod ast;
    pub mod ast_semantic;
    pub mod ats_lowering_resolution;
}

pub use cli::cli_logic::cli_logic;
