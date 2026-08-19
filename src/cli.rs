use std::{env, path::PathBuf};

use crate::{api, commands, yaml_config::append_data};

pub async fn cli_logic() {
    let matches = commands::build_cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let recipe_name = sub_matches
                .get_one::<String>("service")
                .expect("Required argument");

            println!("Sending request for recipe: {}", recipe_name);
            match api::get_recipe(recipe_name, "http://127.0.0.1:3000/services/").await {
                Ok(recipe) => {
                    println!("Successfully retrieved recipe: {:?}", recipe);
                    let target_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                    append_data(&recipe, &target_path);
                },
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
            let url = sub_matches
                .get_one::<String>("service_url")
                .expect("Required argument");
            println!("Getting repository from: {}", url);
        }
        _ => {
                println!("No subcommand provided. Use --help for usage instructions.");
            }
    }
}