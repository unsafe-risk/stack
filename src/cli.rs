use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        name: String,

        #[arg(short, long)]
        go: bool,

        #[arg(short, long)]
        rust: bool,
    },
    Generate {
        path: String,
        name: String,

        #[arg(short, long)]
        protocol: bool,

        #[arg(short, long)]
        message: bool,

        #[arg(short, long)]
        state: bool,

        #[arg(short, long)]
        contract: bool,

        #[arg(short, long)]
        model: bool,

        #[arg(short, long)]
        service: bool,

        #[arg(short, long)]
        mediator: bool,

        #[arg(short, long)]
        aggregator: bool,

        #[arg(short, long)]
        handler: bool,

        #[arg(short, long)]
        adapter: bool,

        #[arg(short, long)]
        server: bool,
    }
}
