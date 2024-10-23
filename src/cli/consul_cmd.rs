use cluster::consul;

pub async fn services() {
    match consul::services::get_services().await {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error fetching services: {}", e),
    }
}

pub async fn service(service_name: &str) {
    match consul::services::get_service(service_name).await {
        Ok(services) => {
            for service in services {
                println!("{}", service)
            }
        },
        Err(e) => eprintln!("Error fetching service: {}", e),
    }
}