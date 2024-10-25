use std::fmt;
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Service {
    #[serde(rename = "ServiceID")]
    pub id: String,

    #[serde(rename = "ServiceName")]
    pub name: String,

    #[serde(rename = "ServiceTags")]
    pub tags: Vec<String>,

    #[serde(rename = "ServiceAddress")]
    pub address: String,

    #[serde(rename = "ServicePort")]
    pub port: u16,
}

// Implementing Display for the Service struct
impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Service {{ id: {}, name: {}, tags: {:?}, address: {}, port: {} }}",
            self.id, self.name, self.tags, self.address, self.port
        )
    }
}

pub async fn get_services(consul_server: &str) -> Result<Value, Error> {
    let url = format!("http://{}/v1/catalog/services", consul_server);
    let response = reqwest::get(url).await?;
    let json = response.json::<Value>().await?;
    Ok(json)
}

pub async fn get_service(consul_server: &str, service_name: &str) -> Result<Vec<Service>, Error> {
    let url = format!("{}/v1/catalog/service/{}", consul_server, service_name);
    let response = reqwest::get(url).await?;
    let json = response.json::<Vec<Service>>().await?;
    Ok(json)
}