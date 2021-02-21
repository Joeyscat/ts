use anyhow::Result;
use structopt::StructOpt;

/// Convert charset
#[derive(StructOpt)]
struct Cli {
    /// The original charset
    original: String,
    /// The target charset to convert
    target: String,
    /// The path to the file to convert
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    Ok(())
}
