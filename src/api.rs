use crate::structs;

pub async fn get_service(url: &str, servicename: &String) -> Result<structs::Recipe, reqwest::Error> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let recipe = response.json::<structs::Recipe>().await?;
    Ok(recipe)
}