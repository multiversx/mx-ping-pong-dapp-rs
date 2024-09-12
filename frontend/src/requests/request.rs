use reqwest::Client;

pub async fn get_request(name: &str, endpoint: &str) -> Result<String, String> {
    let client = Client::new();
    let full_endpoint = format!("{endpoint}/{name}");
    let response = client
        .get(full_endpoint)
        .send()
        .await
        .map_err(|err| format!("{name} request failed: {:?}", err))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(format!("Server error: {:?}", response.status()))
    }
}

pub async fn post_request(name: &str, full_endpoint: &str) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .post(full_endpoint)
        .send()
        .await
        .map_err(|err| format!("{name} request failed: {:?}", err))?;

    if response.status().is_success() {
        let body = response
            .text()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(format!("Server error: {:?}", response.status()))
    }
}
