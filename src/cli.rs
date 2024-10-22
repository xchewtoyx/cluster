use clap::Command;

pub fn build_cli() -> Command {
    Command::new("cluster")
        .about("A CLI app for working with chewcorp cluster resources")
        .subcommand(
            Command::new("consul")
                .about("Commands the deal with consul")
                .subcommand(
                    Command::new("services")
                        .about("List consul services")
                )
                .subcommand(
                    Command::new("service")
                        .about("Show details of consul service")
                        .arg(clap::Arg::new("service-name")
                            .long("service-name")
                            .help("The name of the service to show details for")
                            .required(true))
                )
        )
        .subcommand(Command::new("envoy")
            .about("Commands that deal with envoy")
            .subcommand(
                Command::new("eds")
                    .about("Show details of consul service")
                    .arg(clap::Arg::new("service-name")
                        .long("service-name")
                        .help("The name of the service to show details for")
                        .required(true))
            )

        )
}
