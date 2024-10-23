use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cluster")]
#[command(about = "A CLI app for working with chewcorp cluster resources")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Api {
        /// Show details of consul service
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

#[derive(Subcommand)]
pub enum ConsulCommands {
    /// List consul services
    Services,
    /// Show details of consul service
    Service {
        #[arg(long, help = "The name of the service to show details for")]
        service_name: String,
    },
}

#[derive(Subcommand)]
pub enum EnvoyCommands {
    /// Show details of consul service
    Eds {
        #[arg(long, help = "The name of the service to show details for")]
        service_name: String,
    },
}
