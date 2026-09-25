use reqwest::Client;

pub async fn login(company_url: &str) 
    -> Result<bool, Box<dyn std::error::Error>> {

        let logining = Client::new()
            .post(company_url)
            .header("Accept", "application/json")
            .send()
            .await?
            .error_for_status()?
            .json::<bool>()
            .await?;

        Ok(logining)
}

//change the type of http method