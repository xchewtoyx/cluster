use cluster::consul::services;
use cluster::envoy::eds;
use serde_json;
use std::error::Error;

pub async fn eds(consul_server: &str, service_name: &str) {
    match process_service(consul_server, service_name).await {
        Ok(eds_json) => println!("{}", eds_json),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

async fn process_service(consul_server: &str, service_name: &str) -> Result<String, Box<dyn Error>> {
    let services: Vec<services::Service> = services::get_service(consul_server, service_name)
        .await?;
    let eds_json = eds::transform_consul_to_eds(services)
        .await?;
    Ok(serde_json::to_string(&eds_json)?)
}