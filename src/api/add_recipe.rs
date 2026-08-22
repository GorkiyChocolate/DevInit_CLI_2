use reqwest::Client;

use crate::models::structs::RecipeCompose;

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