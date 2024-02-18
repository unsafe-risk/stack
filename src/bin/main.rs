use core::panic;

use clap::Parser;
use stack::{cli::Cli, config::Config, go};

#[tokio::main]
async fn main() {
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
                Ok(()) => println!("{} config wrote", name),
                Err(e) => println!("Error: {}", e),
            }

            if go {
                match stack::go::init_go_mod(&name).await {
                    Ok(()) => println!("Go mod initialized"),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        stack::cli::Commands::Generate {
            path,
            name,
            protocol,
            message,
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

            match cfg.language {
                stack::config::Language::Go => {
                    let category = if protocol {
                        stack::category::Category::Protocol
                    } else if message {
                        stack::category::Category::Message
                    } else if contract {
                        stack::category::Category::Contract
                    } else if model {
                        stack::category::Category::Model
                    } else if service {
                        stack::category::Category::Service
                    } else if mediator {
                        stack::category::Category::Mediator
                    } else if aggregator {
                        stack::category::Category::Aggregator
                    } else if handler {
                        stack::category::Category::Handler
                    } else if adapter {
                        stack::category::Category::Adapter
                    } else if server {
                        stack::category::Category::Server
                    } else {
                        panic!("You must specify a category")
                    };

                    match go::generate_file(name.as_str(), path.as_str(), &category).await {
                        Ok(()) => println!("{} {} generated", &category, name),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                stack::config::Language::Rust => {
                    println!("Rust not implemented");
                }
            }
        }
    }
}