fn block() -> Vec<crate::Op> {
    let ops = std::iter::from_fn(|| (!matches!(peek(), "|" | ")" | "}" | "]")).then(op));
    ops.flatten().collect()
}

fn op() -> Vec<crate::Op> {
    match (peek(), bump()).0 {
        "(" => paren(),
        "{" => [fork()].into(),
        "[" => [bracket()].into(),
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
        _ => todo!(),
    }
}

fn paren() -> Vec<crate::Op> {
    (block(), assert!(peek() == ")"), bump()).0
}

fn fork() -> crate::Op {
    let bs = std::iter::from_fn(|| {
        (peek() != "}").then(|| (block(), assert!(matches!(peek(), "|" | "}")), bump()).0)
    })
    .collect();
    crate::Op::Fork(bs)
}

fn bracket() -> crate::Op {
    let bs = std::iter::from_fn(|| {
        (peek() != "]").then(|| (block(), assert!(matches!(peek(), "|" | "]")), bump()).0)
    })
    .collect();
    crate::Op::Bracket(bs)
}

fn peek<'src>() -> &'src str {
    todo!()
}

fn bump() {
    todo!()
}
