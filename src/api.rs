use super::structs;
use reqwest::blocking::Client;
use std::error::Error;

pub async fn execute_request(
    client: &Client, // 1. Клиент передается по ссылке
    method: &str,
    url: &str,
    body: Option<&str>,
) -> Result<structs::Recipe, Box<dyn Error>> {
    let mut request_builder = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        _ => return Err(format!("Unsupported HTTP method: {method}").into()),
    };

    if let Some(b) = body {
        // 2. Указываем Content-Type, если передаем тело
        request_builder = request_builder
            .header("Content-Type", "application/json")
            .body(b.to_owned());
    }

    // 3. Отправляем запрос, проверяем статус и парсим JSON
    let response = request_builder.send()?.error_for_status()?;
    let recipe: structs::Recipe = response.json()?;

    Ok(recipe)
}