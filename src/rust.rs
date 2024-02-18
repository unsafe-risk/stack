use std::error::Error;

use tokio::{fs::File, io::AsyncWriteExt, process::Command};

use crate::{category::Category, strfmt::pascal_to_snake};


pub async fn write_struct(writer: &mut File, name: &str) -> Result<(), std::io::Error> {
    writer.write(format!("pub struct {} {{\n}}\n\n", name).as_bytes()).await?;
    writer.write(format!("impl {} {{\n\t", name).as_bytes()).await?;
    writer.write(format!("pub fn new() -> Self {{\n\t\t{} {{}}\n\t}}\n", name).as_bytes()).await?;
    writer.write("}\n".as_bytes()).await?;

    Ok(())
}

pub async fn write_enum(writer: &mut File, name: &str) -> Result<(), std::io::Error> {
    writer.write(format!("pub enum {} {{\n}}\n\n", name).as_bytes()).await?;

    Ok(())
}

pub async fn write_trait(writer: &mut File, name: &str) -> Result<(), std::io::Error> {
    writer.write(format!("pub trait {} {{\n}}\n", name).as_bytes()).await?;

    Ok(())
}

pub async fn write_assembler(writer: &mut File) -> Result<(), std::io::Error> {
    writer.write(format!("#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {{\n\tOk(())\n}}").as_bytes()).await?;

    Ok(())
}

const SERVICE_FOLDER: &str = "src/component/service";
const SERVICE_FILE: &str = "src/component/service/mod.rs";
const MODEL_FOLDER: &str = "src/component/model";
const MODEL_FILE: &str = "src/component/model/mod.rs";
const CONTRACT_FOLDER: &str = "src/component/contract";
const CONTRACT_FILE: &str = "src/component/contract/mod.rs";
const MEDIATOR_FOLDER: &str = "src/controller/mediator";
const MEDIATOR_FILE: &str = "src/controller/mediator/mod.rs";
const AGGREGATOR_FOLDER: &str = "src/controller/aggregator";
const AGGREGATOR_FILE: &str = "src/controller/aggregator/mod.rs";
const HANDLER_FOLDER: &str = "src/controller/handler";
const HANDLER_FILE: &str = "src/controller/handler/mod.rs";
const ADAPTER_FOLDER: &str = "src/controller/adapter";
const ADAPTER_FILE: &str = "src/controller/adapter/mod.rs";
const SERVER_FOLDER: &str = "src/controller/server";
const SERVER_FILE: &str = "src/controller/server/mod.rs";
const MESSAGE_FOLDER: &str = "src/component/message";
const MESSAGE_FILE: &str = "src/component/message/mod.rs";
const PROTOCOL_FOLDER: &str = "src/component/protocol";
const PROTOCOL_FILE: &str = "src/component/protocol/mod.rs";
const STATE_FOLDER: &str = "src/component/state";
const STATE_FILE: &str = "src/component/state/mod.rs";
const ASSEMBLER_FOLDER: &str = "src/bin";

pub async fn init_cargo(name: &str) -> Result<(), std::io::Error> {
    let output = Command::new("cargo")
        .arg("init")
        .arg(".")
        .arg("--lib")
        .output().await?;

    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to init cargo"));
    }

    let cargo_data = tokio::fs::read_to_string("Cargo.toml").await?;
    let lines = cargo_data.split('\n').collect::<Vec<&str>>();
    let mut new_cargo_data = String::new();
    for line in lines {
        if line.starts_with("name") {
            new_cargo_data.push_str(&format!("name = \"{}\"\n", name));
        } else {
            new_cargo_data.push_str(&format!("{}\n", line));
        }
    }

    tokio::fs::write("Cargo.toml", new_cargo_data).await?;

    let output = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("-F")
        .arg("full")
        .output().await?;

    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to add tokio"));
    }

    let output = Command::new("cargo")
        .arg("add")
        .arg("serde")
        .arg("-F")
        .arg("derive")
        .output().await?;

    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to add serde"));
    }

    tokio::fs::create_dir_all(SERVICE_FOLDER).await?;
    File::create(SERVICE_FILE).await?;
    tokio::fs::create_dir_all(MODEL_FOLDER).await?;
    File::create(MODEL_FILE).await?;
    tokio::fs::create_dir_all(CONTRACT_FOLDER).await?;
    File::create(CONTRACT_FILE).await?;
    tokio::fs::create_dir_all(MEDIATOR_FOLDER).await?;
    File::create(MEDIATOR_FILE).await?;
    tokio::fs::create_dir_all(AGGREGATOR_FOLDER).await?;
    File::create(AGGREGATOR_FILE).await?;
    tokio::fs::create_dir_all(HANDLER_FOLDER).await?;
    File::create(HANDLER_FILE).await?;
    tokio::fs::create_dir_all(ADAPTER_FOLDER).await?;
    File::create(ADAPTER_FILE).await?;
    tokio::fs::create_dir_all(SERVER_FOLDER).await?;
    File::create(SERVER_FILE).await?;
    tokio::fs::create_dir_all(MESSAGE_FOLDER).await?;
    File::create(MESSAGE_FILE).await?;
    tokio::fs::create_dir_all(PROTOCOL_FOLDER).await?;
    File::create(PROTOCOL_FILE).await?;
    tokio::fs::create_dir_all(STATE_FOLDER).await?;
    File::create(STATE_FILE).await?;

    let mut mod_file = File::create("src/component/mod.rs").await?;
    mod_file.write("pub mod service;\npub mod model;\npub mod contract;\npub mod message;\npub mod state;\npub mod protocol;\n".as_bytes()).await?;
    let mut mod_file = File::create("src/controller/mod.rs").await?;
    mod_file.write("pub mod aggregator;\npub mod handler;\npub mod adapter;\npub mod server;\npub mod mediator;\n".as_bytes()).await?;

    let mut lib_file = File::create("src/lib.rs").await?;
    lib_file.write("pub mod component;\npub mod controller;\n".as_bytes()).await?;

    Ok(())   
}

pub async fn check_module(prefix: &str, path: &str, name: &str) -> Result<(), std::io::Error> {
    let mut dirs = path.trim_matches('/').split('/').collect::<Vec<&str>>();
    let mut current = prefix.to_string();
    let snake_name = pascal_to_snake(name);

    loop {
        let mod_file = format!("{}/mod.rs", current);
        println!("Checking file: {}", mod_file);
        let mod_data = match tokio::fs::read_to_string(&mod_file).await {
            Ok(f) => f,
            Err(_) => {
                tokio::fs::write(&mod_file, "").await?;
                String::new()
            }
        };

        let v = match dirs.first() {
            Some(v) => *v,
            None => &snake_name,
        };
        
        let mut new_mod_data = String::new();
        if mod_data.len() > 0 {
            new_mod_data.push_str(&mod_data);
        }
        if !mod_data.contains(v) {
            new_mod_data.push_str(&format!("\npub mod {};\n", v));
        }

        tokio::fs::write(&mod_file, new_mod_data).await?;

        if dirs.len() == 0 {
            break;
        }

        let dir = dirs.remove(0);
        current.push_str(&format!("/{}", dir));
    }

    Ok(())
}

pub async fn register_bin(prefix: &str, path: &str, name: &str) -> Result<(), std::io::Error> {
    let snake_name = pascal_to_snake(name);
    let full_path = format!("{}/{}/{}.rs", prefix, path, snake_name);

    let mut cargo_data = tokio::fs::read_to_string("Cargo.toml").await?;

    cargo_data.push_str(&format!("\n[[bin]]\nname = \"{}\"\npath = \"{}\"\n", snake_name, full_path));

    tokio::fs::write("Cargo.toml", cargo_data).await?;

    Ok(())
}

pub async fn generate_file(name: &str, path: &str, _: &str, category: &Category) -> Result<(), Box<dyn Error>> {
    let (prefix, file) = match category {
        Category::Service => (SERVICE_FOLDER ,format!("{}/{}", SERVICE_FOLDER, path)),
        Category::Model => (MODEL_FOLDER ,format!("{}/{}", MODEL_FOLDER, path)),
        Category::Contract => (CONTRACT_FOLDER ,format!("{}/{}", CONTRACT_FOLDER, path)),
        Category::Mediator => (MEDIATOR_FOLDER ,format!("{}/{}", MEDIATOR_FOLDER, path)),
        Category::Aggregator => (AGGREGATOR_FOLDER ,format!("{}/{}", AGGREGATOR_FOLDER, path)),
        Category::Handler => (HANDLER_FOLDER ,format!("{}/{}", HANDLER_FOLDER, path)),
        Category::Adapter => (ADAPTER_FOLDER ,format!("{}/{}", ADAPTER_FOLDER, path)),
        Category::Server => (SERVER_FOLDER ,format!("{}/{}", SERVER_FOLDER, path)),
        Category::Message => (MESSAGE_FOLDER ,format!("{}/{}", MESSAGE_FOLDER, path)),
        Category::Protocol => (PROTOCOL_FOLDER ,format!("{}/{}", PROTOCOL_FOLDER, path)),
        Category::State => (STATE_FOLDER ,format!("{}/{}", STATE_FOLDER, path)),
        Category::Assembler => (ASSEMBLER_FOLDER ,format!("{}/{}", ASSEMBLER_FOLDER, path)),
    };

    tokio::fs::create_dir_all(&file).await?;

    let file = format!("{}/{}.rs", file, pascal_to_snake(name));

    let mut file = File::create(file).await?;

    match category {
        Category::Service => write_struct(&mut file, name).await?,
        Category::Model => write_struct(&mut file, name).await?,
        Category::Contract => write_trait(&mut file, name).await?,
        Category::Mediator => write_struct(&mut file, name).await?,
        Category::Aggregator => write_struct(&mut file, name).await?,
        Category::Handler => write_struct(&mut file, name).await?,
        Category::Adapter => write_struct(&mut file, name).await?,
        Category::Server => write_struct(&mut file, name).await?,
        Category::Message => write_enum(&mut file, name).await?,
        Category::Protocol => write_struct(&mut file, name).await?,
        Category::State => write_enum(&mut file, name).await?,
        Category::Assembler => write_assembler(&mut file).await?,
    }

    match category {
        Category::Assembler => register_bin(prefix, path, name).await?,
        _ => check_module(prefix, path, name).await?,
    }

    Ok(())
}
