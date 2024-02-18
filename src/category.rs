use std::fmt::Display;


#[derive(Debug, PartialEq)]
pub enum Category {
    Service,
    Mediator,
    Aggregator,
    Handler,
    Adapter,
    Server,
    Protocol,
    Message,
    State,
    Contract,
    Model,
    Assembler,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Service => write!(f, "service"),
            Category::Mediator => write!(f, "mediator"),
            Category::Aggregator => write!(f, "aggregator"),
            Category::Handler => write!(f, "handler"),
            Category::Adapter => write!(f, "adapter"),
            Category::Server => write!(f, "server"),
            Category::Protocol => write!(f, "protocol"),
            Category::Message => write!(f, "message"),
            Category::State => write!(f, "state"),
            Category::Contract => write!(f, "contract"),
            Category::Model => write!(f, "model"),
            Category::Assembler => write!(f, "assembler"),
        }
    }
}