use reqwest::Client;

use crate::structs::{ConfigsList, RecipeCompose};

pub async fn add_recipe(
    recipe_name: &str,
    base_url: &str
) -> Result<RecipeCompose, Box<dyn std::error::Error>> {
    let url = format!("{}{}", base_url, recipe_name);

    let recipe = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()?
        .json::<RecipeCompose>()
        .await?;

    Ok(recipe)
}

pub async fn get_config(
    config_url: &str,
    config_name: &str,
) -> Result<ConfigsList, Box<dyn std::error::Error>> {
    let url = format!("{}{}", config_url, config_name);

    let configs_list = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()? // Превращает HTTP 4xx/5xx в Err(reqwest::Error)
        .json::<ConfigsList>()
        .await?;

    Ok(configs_list)
}