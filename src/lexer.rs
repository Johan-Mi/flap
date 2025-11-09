pub fn lex(source_code: &str) -> impl Iterator<Item = &str> {
    source_code
        .lines()
        .map(|line| line.split_once('#').map_or(line, |it| it.0))
        .flat_map(str::split_whitespace)
}
