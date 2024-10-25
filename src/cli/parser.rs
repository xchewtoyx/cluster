
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cluster")]
#[command(about = "A CLI app for working with chewcorp cluster resources")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn run(&self) {
        self.command.run().await
    }
}

// Common arguments shared across commands
#[derive(Parser, Clone)]
pub struct ConsulArgs {
    #[arg(
        long,
        env = "CONSUL_HTTP_ADDR",
        default_value = "http://127.0.0.1:8500",
        help = "Address of the consul server to query"
    )]
    pub consul_server: String,
}


// Define subcommands
#[derive(Subcommand)]
pub enum Commands {
    Api {
        // Start API Server
        #[command(flatten)]
        common: ConsulArgs,
        #[arg(long, help = "Address and port to bind to", default_value = "127.0.0.1:8080")]
        bind: String,
    },
    Consul {
        #[command(subcommand)]
        subcommand: ConsulCommands,
    },
    Envoy {
        #[command(subcommand)]
        subcommand: EnvoyCommands,
    },
}

impl Commands {
    pub async fn run(&self) {
        match self {
            Commands::Api { common, bind } => super::api::serve(&bind, &common.consul_server).await,
            Commands::Consul { subcommand } => subcommand.run().await,
            Commands::Envoy { subcommand } => subcommand.run().await,
        }
    }
}

#[derive(Subcommand)]
pub enum ConsulCommands {
    /// List consul services
    Services {
        #[command(flatten)]
        common: ConsulArgs,
    },
    /// Show details of consul service
    Service {
        #[command(flatten)]
        common: ConsulArgs,
        #[arg(help = "The name of the service to show details for")]
        service_name: String,
    },
}

impl ConsulCommands {
    pub async fn run(&self) {
        match self {
            ConsulCommands::Services { common } => super::consul_cmd::services(&common.consul_server).await,
            ConsulCommands::Service { common, service_name } => super::consul_cmd::service(&common.consul_server, service_name).await,
        }
    }
}

#[derive(Subcommand)]
pub enum EnvoyCommands {
    /// Show details of consul service
    Eds {
        // Consul Server to read details from
        #[command(flatten)]
        common: ConsulArgs,
        // Service name to query
        #[arg(help = "The name of the service to show details for")]
        service_name: String,
    },
}

impl EnvoyCommands {
    pub async fn run(&self) {
        match self {
            EnvoyCommands::Eds { common, service_name } => super::envoy::eds(&common.consul_server, service_name).await,
        }
    }
}
