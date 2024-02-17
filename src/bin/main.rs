use core::panic;

use clap::Parser;
use stack::{cli::Cli, config::Config};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        stack::cli::Commands::Init { name, go, rust } => {
            let c = Config::new(name.clone(), if go {
                stack::config::Language::Go
            } else if rust {
                stack::config::Language::Rust
            } else {
                panic!("You must specify a language")
            });

            match c.write() {
                Ok(()) => println!("{} initialized", name),
                Err(e) => println!("Error: {}", e),
            }
        }
        stack::cli::Commands::Generate {
            path,
            name,
            protocol,
            message,
            state,
            contract,
            model,
            service,
            mediator,
            aggregator,
            handler,
            adapter,
            server,
        } => {
            let cfg = match Config::read() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };
        }
    }
}