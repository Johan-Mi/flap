pub fn lex(source_code: &str) -> impl Iterator<Item = &str> {
    source_code
        .lines()
        .map(|line| line.split_once('#').map_or(line, |it| it.0))
        .flat_map(|mut line| {
            std::iter::from_fn(move || {
                line = line.trim_start();
                (!line.is_empty()).then(|| {
                    let is_array_literal = |c| !matches!(c, '_' | '0'..='9');
                    let (token, l) = line.split_at(
                        std::num::NonZero::new(line.find(is_array_literal).unwrap_or(line.len()))
                            .map_or_else(|| line.ceil_char_boundary(1), std::num::NonZero::get),
                    );
                    line = l;
                    token
                })
            })
        })
}
