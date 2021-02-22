use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::BufRead;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    least: u64,
}


#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(self, least: &u64) {
        let mut s: Vec<(&String, &u64)> = self.0.iter()
            .filter(|&(_k, v)| *v >= *least)
            .collect();
        s.sort_by(|a, b| b.1.cmp(a.1)
            .then(b.0.cmp(a.0)));

        for (k, v) in s {
            println!("{}\t{}", k, v);
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    println!("Processing file: {:?}", &args.path.to_str());
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        for word in line.split(" ") {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display(&args.least);

    Ok(())
}
