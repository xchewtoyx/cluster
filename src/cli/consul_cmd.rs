use cluster::consul;

pub async fn services(consul_server: &str) {
    match consul::services::get_services(consul_server).await {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error fetching services: {}", e),
    }
}

pub async fn service(consul_server: &str, service_name: &str) {
    match consul::services::get_service(consul_server, service_name).await {
        Ok(services) => {
            for service in services {
                println!("{}", service)
            }
        },
        Err(e) => eprintln!("Error fetching service: {}", e),
    }
}