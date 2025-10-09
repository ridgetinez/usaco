// Using fully qualified names so I don't get errors about double imports
// when this gets !included in into a single submit file.
pub fn parse_line<T: std::str::FromStr + Default>(
    lines: &mut impl Iterator<Item = std::io::Result<String>>,
) -> std::io::Result<Vec<T>> {
    Ok(lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse().unwrap_or_default())
        .collect::<Vec<T>>())
}
