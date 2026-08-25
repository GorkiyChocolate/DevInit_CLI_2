use crate::{api, cli, file_config};
use std::{env, path::PathBuf};

use api::add_recipe::add_recipe;
use api::get_config::get_config;
use cli::commands;
use file_config::env_config::env_file_config;
use file_config::yaml_config::{yaml_configs_data, yaml_data};

pub async fn cli_logic() {
    let base_url = "http://127.0.0.1:3000/services/";
    let configs_url = "http://127.0.0.1:3000/configs/";
    let matches = commands::build_cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let recipe_name = sub_matches
                .get_one::<String>("service")
                .expect("Required argument");

            println!("Sending request for recipe: {}", recipe_name);
            match add_recipe(recipe_name, base_url).await {
                Ok(recipe) => {
                    println!("Successfully retrieved recipe: {:?}", recipe);
                    let target_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("compose.yaml.example");
                    let env_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("env.example");

                    if let Err(e) = yaml_data(&recipe, &target_path) {
                        eprintln!("Error updating {}: {}", target_path.display(), e);
                    }
                    if let Err(e) = env_file_config(&recipe, &env_path) {
                        eprintln!("Error updating {}: {}", env_path.display(), e);
                    }
                }
                Err(e) => eprintln!("Error fetching recipe: {}", e),
            }
        }

        Some(("list", sub_matches)) => {
            let service_type = sub_matches.get_one::<String>("type");
            let page = sub_matches.get_one::<String>("page");

            println!(
                "Fetching list... Type: {:?}, Page: {:?}",
                service_type, page
            );
        }

        Some(("get", sub_matches)) => {
            let config_name = sub_matches
                .get_one::<String>("service_url")
                .expect("Required argument");
            println!("Getting repository from: {}", configs_url);

            match get_config(configs_url, config_name).await {
                Ok(configs_list) => {
                    println!("Succesfully retrieved configs: {:?}", configs_list);
                    let target_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("compose.yaml.example");

                    if let Err(e) = yaml_configs_data(&configs_list, &target_path) {
                        eprintln!("Error updating {}: {}", target_path.display(), e);
                    }
                }
                Err(e) => eprintln!("Error fetching config: {}", e),
            }
        }

        Some(("login", sub_matches)) => {
            let config_name = sub_matches
                .get_one::<String>("login_url")
                .expect("Required argument");
            println!("logging in: {}", configs_url);

        }
        _ => {
            println!("No subcommand provided. Use --help for usage instructions.");
        }
    }
}
