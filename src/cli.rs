use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new stack project
    Init {
        /// A name for the project
        name: String,

        /// The language to use for the project (Go)
        #[arg(short, long)]
        go: bool,

        /// The language to use for the project (Rust)
        #[arg(short, long)]
        rust: bool,
    },
    /// Generate a new stack component
    Generate {
        /// The path to the component to generate. Must use '/' as a separator
        path: String,

        /// The name of the component to generate. Recommended to use PascalCase
        name: String,

        /// A component to validate data, data transmission, or data transformation
        #[arg(short, long)]
        protocol: bool,

        /// A data structure that represents a message between serveral components
        #[arg(short = 'g', long)]
        message: bool,

        /// A component that defines a contract (interface / trait) for handlers
        #[arg(short, long)]
        contract: bool,

        /// A data structure that represents a transferable object
        #[arg(short = 'd', long)]
        model: bool,

        /// A component that provides some functionality to other components
        #[arg(short, long)]
        service: bool,

        /// A component that mediates between services
        #[arg(short, long)]
        mediator: bool,

        /// A component that aggregates data from services
        #[arg(short = 'r', long)]
        aggregator: bool,

        /// A component that handles business logic
        #[arg(short = 'n', long)]
        handler: bool,

        /// A component that adapts data from server to handler
        #[arg(short, long)]
        adapter: bool,

        /// A component that listens for requests and sends responses to clients
        #[arg(short = 'v', long)]
        server: bool,

        /// A component that assembles data from services. Just executable code
        #[arg(short = 'b', long)]
        assembler: bool,
    }
}
