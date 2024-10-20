use clap::ArgMatches;

pub async fn run(matches: &ArgMatches) {
    println!("Running envoy with {:?}", matches);
}
