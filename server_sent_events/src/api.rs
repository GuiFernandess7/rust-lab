use reqwest;

pub async fn call_endpoint(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        println!("Response: {}", body);
        return body;
    } else {
        println!("Error: {}", response.status());
        return String::from(format!("Error calling API, status: {}", response.status()));
    }
}
