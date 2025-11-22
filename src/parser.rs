use crate::{Op, ast};

pub fn parse<'src>(tokens: impl Iterator<Item = &'src str>) -> ast::Tree {
    let finish_modifier = |b: &mut ast::Builder| {
        if matches!(b.parent(), Op::Fold | Op::Scan) {
            b.finish_node();
        }
    };

    let primitive = |b: &mut ast::Builder, op| {
        b.start_node(op);
        b.finish_node();
        finish_modifier(b);
    };

    let mut b = ast::Builder::default();
    b.start_node(Op::Block);
    for token in tokens {
        match token {
            "(" => b.start_node(Op::Block),
            "{" => {
                b.start_node(Op::Fork);
                b.start_node(Op::Block);
            }
            "[" => {
                b.start_node(Op::Bracket);
                b.start_node(Op::Block);
            }
            ")" => {
                b.finish_node();
                finish_modifier(&mut b);
            }
            "}" | "]" => {
                b.finish_node();
                b.finish_node();
                finish_modifier(&mut b);
            }
            "|" => {
                b.finish_node();
                b.start_node(Op::Block);
            }
            "/" => b.start_node(Op::Fold),
            "\\" => b.start_node(Op::Scan),
            "+" => primitive(&mut b, Op::Add),
            "-" => primitive(&mut b, Op::Sub),
            "×" => primitive(&mut b, Op::Mul),
            "↧" => primitive(&mut b, Op::Min),
            "↥" => primitive(&mut b, Op::Max),
            "<" => primitive(&mut b, Op::Lt),
            "≤" => primitive(&mut b, Op::Le),
            "=" => primitive(&mut b, Op::Eq),
            "@" => primitive(&mut b, Op::Select),
            "▽" => primitive(&mut b, Op::Keep),
            "," => primitive(&mut b, Op::Join),
            "⧻" => primitive(&mut b, Op::Length),
            "⍳" => primitive(&mut b, Op::Iota),
            "⇌" => primitive(&mut b, Op::Reverse),
            "⍏" => primitive(&mut b, Op::Rise),
            "⍖" => primitive(&mut b, Op::Fall),
            "·" => primitive(&mut b, Op::Id),
            "○" => primitive(&mut b, Op::Pop),
            s => {
                b.start_node(Op::Push);
                for n in s.split('_') {
                    b.start_node(Op::Number(n.parse().unwrap()));
                    b.finish_node();
                }
                b.finish_node();
                finish_modifier(&mut b);
            }
        }
    }
    b.finish_node();
    b.build()
}
