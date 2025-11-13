type Parser<'a, 'src> = std::iter::Peekable<&'a mut dyn Iterator<Item = &'src str>>;

pub fn block(p: &mut Parser) -> Vec<crate::Op> {
    let f = || (!matches!(p.peek().copied(), None | Some("|" | ")" | "}" | "]"))).then(|| op(p));
    std::iter::from_fn(f).flatten().collect()
}

fn op(p: &mut Parser) -> Vec<crate::Op> {
    let variadic_modifier = |p: &mut Parser, op: fn(_) -> _, end| {
        let bs = std::iter::from_fn(|| {
            let b = (p.peek() != Some(&end)).then(|| block(p));
            b.inspect(|_| assert!(p.peek() == Some(&end) || p.next() == Some("|")))
        });
        [op(bs.collect())].into()
    };

    match p.next().unwrap() {
        "(" => (block(p), assert_eq!(p.next(), Some(")"))).0,
        "{" => variadic_modifier(p, crate::Op::Fork, "}"),
        "[" => variadic_modifier(p, crate::Op::Bracket, "]"),
        "/" => [crate::Op::Fold(op(p))].into(),
        "\\" => [crate::Op::Scan(op(p))].into(),
        "+" => [crate::Op::Add].into(),
        "-" => [crate::Op::Sub].into(),
        "×" => [crate::Op::Mul].into(),
        "↧" => [crate::Op::Min].into(),
        "↥" => [crate::Op::Max].into(),
        "<" => [crate::Op::Lt].into(),
        "≤" => [crate::Op::Le].into(),
        "=" => [crate::Op::Eq].into(),
        "@" => [crate::Op::Select].into(),
        "▽" => [crate::Op::Keep].into(),
        "," => [crate::Op::Join].into(),
        "⧻" => [crate::Op::Length].into(),
        "⍳" => [crate::Op::Iota].into(),
        "⇌" => [crate::Op::Reverse].into(),
        "⍏" => [crate::Op::Rise].into(),
        "⍖" => [crate::Op::Fall].into(),
        "·" => [crate::Op::Id].into(),
        "○" => [crate::Op::Pop].into(),
        s => [crate::Op::Push(
            s.split('_').map(|it| it.parse().unwrap()).collect(),
        )]
        .into(),
    }
}
