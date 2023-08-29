use std::{fs::{File, self}, env, path::Path, io::Write};

mod args;
mod util;

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use util::*;

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
    let gen = PackageGen::new(&args.path, args.names.clone());
    match gen.default_new() {
        Ok(msg) => log::log!(log::Level::Info, "{}", msg),
        Err(e) => { log::error!("{}", e); return; },
    }

    if !args.names.is_empty() {
        match gen.create_license() {
            Ok(msg) => log::log!(log::Level::Info, "{}", msg),
            Err(e) => { log::error!("{}", e); return; },
        }
    }

    
    if !args.readme {
        match gen.create_readme() {
            Ok(msg) => log::log!(log::Level::Info, "{}", msg),
            Err(e) => { log::error!("{}", e); return; },
        }
    }

}

struct PackageGen {
    path: String,
    package_name: String,
    year: String,
    names: String,
    cargo_toml: Vec<String>,
}

impl PackageGen {
    fn new(path_string: &String, names: String) -> Self {
        let path = Path::new(path_string);
        let package_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let year = chrono::Utc::now().format("%Y").to_string();
        let cargo_toml = cargotoml(package_name.clone());
        Self { path: path_string.clone(), package_name, year, names, cargo_toml }
    }

    fn default_new(&self) -> std::io::Result<String> {
        // init git repo
        // create src/main.rs
        // create Cargo.toml with defaults
        // create .gitignore

        // create path
        let path = Path::new(&self.path);

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
        File::create("Cargo.toml")?.write_all(self.cargo_toml.join("\n").as_bytes())?;

        // create .gitignore
        File::create(".gitignore")?.write_all("/target\n".as_bytes())?;

        Ok(format!("Created {}", path.display()))
    }

    fn create_readme(&self) -> std::io::Result<String> {
        let path = Path::new(&self.path);

        // create README.md
        let readme = format!("# {}", self.package_name);
        File::create("README.md")?.write_all(readme.as_bytes())?;

        // Add README.md to Cargo.toml
        let empty_position = self.cargo_toml.iter().position(|s| s.is_empty()).unwrap();
        // self.cargo_toml.insert(empty_position, "readme = \"README.md\"");
        let mut cargo_toml = self.cargo_toml.clone();
        cargo_toml.insert(empty_position, "readme = \"README.md\"".to_string());
        File::create("Cargo.toml")?.write_all(cargo_toml.join("\n").as_bytes())?;

        Ok(format!("Created {}", path.display()))
    }

    fn create_license(&self) -> std::io::Result<String> {
        let path = Path::new(&self.path);
        
        // create LICENSE
        File::create("LICENSE")?.write_all(mitlicense(&self.names, &self.year).as_bytes())?;

        // Add LICENSE to Cargo.toml
        let empty_position = self.cargo_toml.iter().position(|s| s.is_empty()).unwrap();
        // self.cargo_toml.insert(empty_position, "license = \"MIT\"");
        let mut cargo_toml = self.cargo_toml.clone();
        cargo_toml.insert(empty_position, "license = \"MIT\"".to_string());
        File::create("Cargo.toml")?.write_all(cargo_toml.join("\n").as_bytes())?;

        Ok(format!("Created {}", path.display()))
    }
}