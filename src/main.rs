mod cli;
use clap::Parser;

#[tokio::main]
async fn main() {
    let app = cli::parser::Cli::parse();
    app.run().await;
}
