use anyhow::{Context, Result, Error};
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

    // TODO use BufReader instead of read_to_string()
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    ts::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
