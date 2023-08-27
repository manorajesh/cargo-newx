use std::{fs::{File, self}, env, path::Path, io::Write};

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

mod args;

fn main() {
    let args = args::Args::parse();

    // Map verbosity count to log level
    let log_level = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    // Set up logging
    Builder::new()
        .filter(None, log_level)
        .default_format()
        .init();

    // Create new package
    let path = Path::new(&args.path);
    match default_new(path) {
        Ok(msg) => log::log!(log::Level::Info, "{}", msg),
        Err(e) => log::error!("{}", e),
    }

}

fn default_new(path: &Path) -> std::io::Result<String> {
    // create src/main.rs
    // create Cargo.toml with defaults
    // create .gitignore
    // init git repo

    // check if path exists
    if path.exists() {
        // check if path is empty
        if !is_empty_or_dir(&path)? {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("{} is not empty", path.display())));
        }
    }

    // create path and change to it
    // init git repo
    git2::Repository::init(&path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    env::set_current_dir(&path)?;  

    // create src/main.rs
    // create src directory
    fs::create_dir_all("src")?;

    // create src/main.rs
    File::create("src/main.rs")?.write_all(helloworld_rs().as_bytes())?;

    // create Cargo.toml with defaults
    let name = path.file_name().unwrap().to_str().unwrap();
    let cargo_toml = cargotoml(name.to_string());
    File::create("Cargo.toml")?.write_all(cargo_toml.as_bytes())?;

    // create .gitignore
    File::create(".gitignore")?.write_all("/target\n".as_bytes())?;

    Ok(format!("Created {}", path.display()))
}

fn is_empty_or_dir(path: &Path) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        let entries = fs::read_dir(path)?;
        Ok(entries.count() == 0)
    } else {
        Ok(false)
    }
}

fn cargotoml(name: String) -> String {
    format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
    
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    
[dependencies]
    "#, name)
}

fn helloworld_rs() -> String {
r#"fn main() {
    println!("Hello, world!");
}"#.to_string()
}

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T> LogExpect<T> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                log::error!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}

impl<T, E> LogExpect<T> for Result<T, E> 
where E: std::fmt::Display
{
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                if msg.is_empty() {
                    log::error!("{}", e);
                } else {
                    log::error!("{}", msg);
                    log::error!("{}", e);
                }
                
                std::process::exit(1);
            }
        }
    }
}