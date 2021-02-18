use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write)
                    -> std::io::Result<()> {
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

// #[test]
// fn find_a_match() {
//     let mut file = NamedTempFile::new().unwrap();
//     writeln!(file, "A test\nActual content\nMore content\nAnother test").unwrap();
//
//     let f = File::open(file.path()).unwrap();
//     let reader = BufReader::new(f);
//
//     let mut result = Vec::new();
//     let r = find_matches(reader, "test", &mut result);
//     assert_eq!(result, b"A test\nAnother test\n");
//     println!("r-------{:?}", r);
// }
