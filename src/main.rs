mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("consul", sub_m)) => commands::consul_cmd::run(sub_m).await,
        Some(("envoy", sub_m)) => commands::envoy::run(sub_m).await,
        _ => eprintln!("Unknown command. Use --help for more info."),
    }
}