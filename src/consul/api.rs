use reqwest::Error;

pub async fn get_services() -> Result<String, Error> {
    let url = "http://localhost:8500/v1/catalog/services";
    let response = reqwest::get(url).await?;
    let json = response.text().await?;
    Ok(json)
}
