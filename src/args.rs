use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Create a new cargo package at <path> with options",
)]
pub struct Args {
    /// Path to the new package
    pub path: String,

    /// Verbosity level
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Create Apache LICENSE
    /// TODO: Implement
    
    /// Create README.md [default: true]
    #[clap(short, long, action)]
    pub readme: bool,

    /// Create MIT LICENSE with names
    #[clap(short, long, default_value = "<copyright holders>")]
    pub names: String,
}