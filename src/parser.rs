fn block() -> Vec<crate::Op> {
    std::iter::from_fn(|| (!matches!(peek(), "|" | "}" | "]")).then(op)).collect()
}

fn op() -> crate::Op {
    match (peek(), bump()).0 {
        "{" => fork(),
        "[" => bracket(),
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
        _ => todo!(),
    }
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
