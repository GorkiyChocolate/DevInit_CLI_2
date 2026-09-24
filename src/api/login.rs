use reqwest::Client;

pub async fn login(company_url: &str) 
    -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}",company_url);

        let logining = Client::new()
            .post(&url)
            .header("Accept", "application/json")
            .send()
            .await?
            .error_for_status()?
            .json::<bool>()
            .await?;

        Ok(logining)
}

//change the type of http method