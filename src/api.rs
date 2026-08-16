use crate::structs::RecipeCompose;


pub fn get_recipe(recipe_name: &str, url: &str) -> Result<RecipeCompose, ureq::Error> {

    let url = format!("{}{}",url, recipe_name);

    let response = ureq::get(&url)
        .set("Accept", "application/json")
        .call()?;

    let recipe: RecipeCompose = response.into_json()?;

    Ok(recipe)
}