use::serde::{Deserialize, Serialize};

pub const CONFIG_FILE: &str = "stack.yaml";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub language: Language,
}

impl Config {
    pub fn new(name: String, language: Language) -> Self {
        Self { name, language }
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(CONFIG_FILE)?;
        match serde_yaml::to_writer(file, self) {
            Ok(()) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn read() -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(CONFIG_FILE)?;
        match serde_yaml::from_reader(file) {
            Ok(c) => Ok(c),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Language {
    Go,
    Rust,
}