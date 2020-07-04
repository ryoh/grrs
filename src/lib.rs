pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> anyhow::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

#[test]
fn find_a_match() -> anyhow::Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");

    Ok(())
}
