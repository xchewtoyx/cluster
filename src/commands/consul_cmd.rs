use clap::ArgMatches;
use cluster::consul;

pub async fn run(matches: &ArgMatches) {
    println!("Running consul with {:?}", matches);
    if let Some(("services", _)) = matches.subcommand() {
        match consul::api::get_services().await {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Error fetching services: {}", e),
        }
    }
}
