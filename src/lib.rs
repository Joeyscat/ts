pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write)
                    -> std::io::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let r = find_matches("hello world\nhello jojo", "jojo", &mut result);
    assert_eq!(result, b"hello jojo\n");
    println!("r-------{:?}", r);
}
