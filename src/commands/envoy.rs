use clap::ArgMatches;
use cluster::consul::api;
use cluster::envoy::eds;
use std::error::Error;

pub async fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("eds", sub_m)) => {
            if let Some(service_name) = sub_m.get_one::<String>("service-name") {
                if let Err(e) = process_service(service_name).await {
                    eprintln!("Error: {:?}", e);
                }
            } else {
                eprintln!("Error: --service-name argument is required");
            }
        }
        _ => {}
    }
}

async fn process_service(service_name: &str) -> Result<String, Box<dyn Error>> {
    let services: Vec<api::Service> = api::get_service(service_name)
        .await?;

    let eds_json = eds::transform_consul_to_eds(services)
        .await?;

    println!("{}", eds_json);    
    Ok(eds_json)
}