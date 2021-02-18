use std::fs::File;
use std::io::{BufReader};

use anyhow::{ Error, Result};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    if args.pattern.is_empty() {
        return Err(Error::msg( "argument <pattern> must not be empty"));
    };

    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    ts::find_matches(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
