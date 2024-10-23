use clap::Parser;

mod cli;
use cli::parser::{Cli, Commands, ConsulCommands, EnvoyCommands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Consul { subcommand } => match subcommand {
            ConsulCommands::Services => cli::consul_cmd::services().await,
            ConsulCommands::Service { service_name } => cli::consul_cmd::service(&service_name).await,
        },
        Commands::Envoy { subcommand } => match subcommand {
            EnvoyCommands::Eds { service_name } => cli::envoy::eds(&service_name).await,
        },
        Commands::Api { bind } => cli::api::serve(&bind).await,
    }
}
