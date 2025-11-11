fn block() -> Vec<crate::Op> {
    std::iter::from_fn(|| (!matches!(peek(), "|" | "}" | "]")).then(op)).collect()
}

fn op() -> crate::Op {
    match (peek(), bump()).0 {
        "{" => fork(),
        "[" => bracket(),
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
