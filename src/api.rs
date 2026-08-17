use crate::structs::RecipeCompose;

pub async fn get_recipe(recipe_name: &str, base_url: &str) -> Result<RecipeCompose, Box<dyn std::error::Error>> {
    let url = format!("{}{}", base_url, recipe_name);

    let recipe = reqwest::Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<RecipeCompose>()
        .await?;

    Ok(recipe)
}