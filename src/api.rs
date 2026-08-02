use crate::structs;

pub async fn add_service(service_name: &str) -> Result<structs::Recipe, reqwest::Error> {
    let url = format!("http://localhost:3000:{}", service_name);
    let response = reqwest::get(url).await?.error_for_status()?;
    let recipe = response.json::<structs::Recipe>().await?;
    Ok(recipe)
}

pub async fn get_service(repo_name: &str) -> Result<structs::Recipe, reqwest::Error> {
    let url = format!("http://localhost:3000:{}", repo_name);
    let response = reqwest::get(url).await?.error_for_status()?;
    let recipe = response.json::<structs::Recipe>().await?;
    Ok(recipe)
}