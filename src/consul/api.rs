use reqwest::Error;

pub async fn get_services() -> Result<String, Error> {
    let url = "http://localhost:8500/v1/catalog/services";
    let response = reqwest::get(url).await?;
    let json = response.text().await?;
    Ok(json)
}

pub async fn get_service(service_name: &str) -> Result<String, Error> {
    let url = format!("http://localhost:8500/v1/catalog/service/{}", service_name);
    let response = reqwest::get(url).await?;
    let json = response.text().await?;
    Ok(json)
}