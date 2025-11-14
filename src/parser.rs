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
        op(bs.collect())
    };

    Vec::from([match p.next().unwrap() {
        "(" => return (block(p), assert_eq!(p.next(), Some(")"))).0,
        "{" => variadic_modifier(p, crate::Op::Fork, "}"),
        "[" => variadic_modifier(p, crate::Op::Bracket, "]"),
        "/" => crate::Op::Fold(op(p)),
        "\\" => crate::Op::Scan(op(p)),
        "+" => crate::Op::Add,
        "-" => crate::Op::Sub,
        "×" => crate::Op::Mul,
        "↧" => crate::Op::Min,
        "↥" => crate::Op::Max,
        "<" => crate::Op::Lt,
        "≤" => crate::Op::Le,
        "=" => crate::Op::Eq,
        "@" => crate::Op::Select,
        "▽" => crate::Op::Keep,
        "," => crate::Op::Join,
        "⧻" => crate::Op::Length,
        "⍳" => crate::Op::Iota,
        "⇌" => crate::Op::Reverse,
        "⍏" => crate::Op::Rise,
        "⍖" => crate::Op::Fall,
        "·" => crate::Op::Id,
        "○" => crate::Op::Pop,
        s => crate::Op::Push(s.split('_').map(|it| it.parse().unwrap()).collect()),
    }])
}
