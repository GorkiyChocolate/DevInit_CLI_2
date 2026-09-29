use reqwest::blocking::Client;

pub fn login(company_url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let logining = Client::new()
        .post(company_url)
        .header("Accept", "application/json")
        .send()?
        .error_for_status()?
        .json::<bool>()?;

    Ok(logining)
}
