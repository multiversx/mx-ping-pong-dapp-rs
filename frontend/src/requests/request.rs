use reqwest::Client;
use serde_json::{json, Value};

pub async fn get_request(name: &str, full_endpoint: &str) -> Result<Value, Value> {
    let client = Client::new();
    let response = client
        .get(full_endpoint)
        .send()
        .await
        .map_err(|err| format!("{name} request failed: {:?}", err))?;

    if response.status().is_success() {
        let body = response
            .json()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(json!(format!("Server error: {:?}", response.status())))
    }
}

pub async fn post_request(
    name: &str,
    full_endpoint: &str,
    req_body: Option<&Value>,
) -> Result<Value, Value> {
    let client = Client::new();

    let request = client.post(full_endpoint);

    let response = if let Some(body) = req_body {
        request.json(body)
    } else {
        request
    };

    let response = response
        .send()
        .await
        .map_err(|err| format!("{name} request failed: {:?}", err))?;

    if response.status().is_success() {
        let body = response
            .json()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(json!({
            "error": format!("Server error: {:?}", response.status())
        }))
    }
}
