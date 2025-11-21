type Parser<'a, 'src> = std::iter::Peekable<&'a mut dyn Iterator<Item = &'src str>>;

pub fn block(p: &mut Parser, literals: &mut Vec<i32>) -> Vec<crate::Op> {
    let f = || {
        (!matches!(p.peek().copied(), None | Some("|" | ")" | "}" | "]"))).then(|| op(p, literals))
    };
    std::iter::from_fn(f).flatten().collect()
}

fn op(p: &mut Parser, literals: &mut Vec<i32>) -> Vec<crate::Op> {
    let variadic_modifier = |p: &mut Parser, literals: &mut _, op: fn(_) -> _, end| {
        let bs = std::iter::from_fn(|| {
            let b = (p.peek() != Some(&end)).then(|| block(p, literals));
            b.inspect(|_| assert!(p.peek() == Some(&end) || p.next() == Some("|")))
        });
        op(bs.collect())
    };

    Vec::from([match p.next().unwrap() {
        "(" => return (block(p, literals), assert_eq!(p.next(), Some(")"))).0,
        "{" => variadic_modifier(p, literals, crate::Op::Fork, "}"),
        "[" => variadic_modifier(p, literals, crate::Op::Bracket, "]"),
        "/" => crate::Op::Fold(op(p, literals)),
        "\\" => crate::Op::Scan(op(p, literals)),
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
        s => {
            let start = literals.len();
            literals.extend(s.split('_').map(|it| it.parse::<i32>().unwrap()));
            crate::Op::Push(start..literals.len())
        }
    }])
}
