use clap::ArgMatches;
use cluster::consul;

pub async fn run(matches: &ArgMatches) {
    println!("Running consul with {:?}", matches);
    match matches.subcommand() {
        Some(("services", _)) => {
            match consul::api::get_services().await {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Error fetching services: {}", e),
            }
        }
        Some(("service", sub_m)) => {
            if let Some(service_name) = sub_m.get_one::<String>("service-name") {
                match consul::api::get_service(service_name).await {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Error fetching service: {}", e),
                }
            } else {
                eprintln!("Error: --service-name argument is required");
            }
        }
        _ => {}
    }
}