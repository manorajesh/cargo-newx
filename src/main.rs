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
    let gen = PackageGen::new(&args.path, args.names);
    match gen.default_new() {
        Ok(msg) => log::log!(log::Level::Info, "{}", msg),
        Err(e) => log::error!("{}", e),
    }

}

struct PackageGen {
    path: Box<Path>,
    package_name: String,
    year: String,
    names: String,
}

impl PackageGen {
    fn new(path: &'static String, names: String) -> Self {
        let path = Box::new(Path::new(&path));
        let package_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let year = chrono::Utc::now().format("%Y").to_string();
        Self { path, package_name, year, names }
    }

    fn default_new(&self) -> std::io::Result<String> {
        // init git repo
        // create src/main.rs
        // create Cargo.toml with defaults
        // create .gitignore

        // check if path exists
        if self.path.exists() {
            // check if path is empty
            if !is_empty_or_dir(&self.path)? {
                return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!("{} is not empty", self.path.display())));
            }
        }

        // create path and change to it
        // init git repo
        git2::Repository::init(&self.path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        env::set_current_dir(&self.path)?;  

        // create src/main.rs
        // create src directory
        fs::create_dir_all("src")?;

        // create src/main.rs
        File::create("src/main.rs")?.write_all(helloworld_rs().as_bytes())?;

        // create Cargo.toml with defaults
        let cargo_toml = cargotoml(self.package_name.to_string());
        File::create("Cargo.toml")?.write_all(cargo_toml.as_bytes())?;

        // create .gitignore
        File::create(".gitignore")?.write_all("/target\n".as_bytes())?;

        Ok(format!("Created {}", self.path.display()))
    }

    fn create_readme(&self) -> std::io::Result<String> {
        // create README.md
        let readme = format!("# {}", self.package_name);
        File::create("README.md")?.write_all(readme.as_bytes())?;
        Ok(format!("Created {}", self.path.display()))
    }

    fn create_license(&self) -> std::io::Result<String> {
        // create LICENSE
        File::create("LICENSE")?.write_all(mitlicense(self.names, self.year).as_bytes())?;
        Ok(format!("Created {}", self.path.display()))
    }
}