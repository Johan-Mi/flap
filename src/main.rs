mod ast;
mod lexer;
mod parser;

fn main() {
    let mut source_code = String::new();
    let _: usize = std::io::Read::read_to_string(&mut std::io::stdin(), &mut source_code).unwrap();
    let tree = parser::parse(lexer::lex(&source_code));
    let mut s = Vec::new();
    x(tree.root(), &mut s);
    for s in s {
        println!("{s:?}");
    }
}

#[derive(Clone, Copy)]
enum Op {
    Block,
    Number(i32),
    /// `1_2_3`
    Push,
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `×`
    Mul,
    /// `↧`
    Min,
    /// `↥`
    Max,
    /// `<`
    Lt,
    /// `≤`
    Le,
    /// `=`
    Eq,
    /// `@`
    Select,
    /// `▽`
    Keep,
    /// `,`
    Join,
    /// `⧻`
    Length,
    /// `⍳`
    Iota,
    /// `⇌`
    Reverse,
    /// `⍏`
    Rise,
    /// `⍖`
    Fall,
    /// `/f`
    Fold,
    /// `\f`
    Scan,
    /// `·`
    Id,
    /// `○`
    Pop,
    /// `{f|g|h}`
    Fork,
    /// `[f|g|h]`
    Bracket,
}

fn x(node: crate::ast::Node, s: &mut Vec<Vec<i32>>) {
    match node.op() {
        Op::Block => {
            for child in node.children() {
                x(child, s);
            }
        }
        Op::Number(_) => unreachable!(),
        Op::Push => {
            let ns = node.children().map(|it| match it.op() {
                Op::Number(n) => n,
                _ => unreachable!(),
            });
            s.push(ns.collect());
        }
        Op::Add => p2(s, std::ops::Add::add),
        Op::Sub => p2(s, std::ops::Sub::sub),
        Op::Mul => p2(s, std::ops::Mul::mul),
        Op::Min => p2(s, Ord::min),
        Op::Max => p2(s, Ord::max),
        Op::Lt => p2(s, |a, b| (a < b).into()),
        Op::Le => p2(s, |a, b| (a <= b).into()),
        Op::Eq => p2(s, |a, b| (a == b).into()),
        Op::Select => match g(s) {
            [a, b] => s.push(b.iter().map(|&i| a[usize::try_from(i).unwrap()]).collect()),
        },
        Op::Keep => {
            let [a, b] = g(s);
            let f = |(a, b)| std::iter::repeat_n(a, usize::try_from(b).unwrap());
            s.push(c(a, b).flat_map(f).collect());
        }
        Op::Join => match g(s) {
            [mut a, b] => (a.extend(b), s.push(a)).1,
        },
        Op::Length => match g(s) {
            [a] => s.push(Vec::from([a.len().try_into().unwrap()])),
        },
        Op::Iota => {
            let [i] = g(s);
            let [i] = i.try_into().unwrap();
            s.push((0..i).collect());
        }
        Op::Reverse => s.last_mut().unwrap().reverse(),
        Op::Rise => grade_by(s, std::convert::identity),
        Op::Fall => grade_by(s, std::cmp::Reverse),
        Op::Fold => {
            let f = node.children().next().unwrap();
            let [v] = g(s);
            for a in v {
                s.push([a].into());
                x(f, s);
            }
        }
        Op::Scan => {
            let f = node.children().next().unwrap();
            let [v] = g(s);
            let mut w = Vec::with_capacity(v.len());
            if let [init, v @ ..] = &*v {
                w.push(*init);
                s.push(Vec::from([*init]));
                for &a in v {
                    s.push(Vec::from([a]));
                    x(f, s);
                    w.extend(s.last().unwrap());
                }
                let _: Vec<i32> = s.pop().unwrap();
            }
            s.push(w);
        }
        Op::Id => {}
        Op::Pop => _ = s.pop().unwrap(),
        Op::Fork => {
            let mut v = Vec::new();
            for f in node.children() {
                let (i, _) = s_(f);
                v.extend(s[s.len() - i..].iter().cloned());
                x(f, &mut v);
            }
            s.truncate(s.len() - s_(node).0);
            s.extend(v);
        }
        Op::Bracket => {
            let mut v = Vec::new();
            for f in node.children() {
                let (_, o) = s_(f);
                x(f, s);
                v.extend(s.drain(s.len() - o..));
            }
            s.extend(v);
        }
    }
}

fn grade_by<K: Ord>(s: &mut Vec<Vec<i32>>, f: fn(i32) -> K) {
    let [a] = g(s);
    let mut i: Vec<_> = (0..a.len().try_into().unwrap()).collect();
    i.sort_by_key(|&i| f(a[usize::try_from(i).unwrap()]));
    s.push(i);
}

fn g<const N: usize>(s: &mut Vec<Vec<i32>>) -> [Vec<i32>; N] {
    s.split_off(s.len() - N).try_into().unwrap()
}

fn p2(s: &mut Vec<Vec<i32>>, f: fn(i32, i32) -> i32) {
    let [a, b] = g(s);
    s.push(c(a, b).map(|(a, b)| f(a, b)).collect());
}

fn c(a: Vec<i32>, b: Vec<i32>) -> impl Iterator<Item = (i32, i32)> {
    let len = a.len().max(b.len());
    a.into_iter().cycle().zip(b.into_iter().cycle()).take(len)
}

fn s_(node: crate::ast::Node) -> (usize, usize) {
    let fork = |(i1, o1), (i2, o2)| (usize::max(i1, i2), o1 + o2);
    let bracket = |(i1, o1), (i2, o2)| (i1 + i2, o1 + o2);
    let mut children = node.children();

    match node.op() {
        Op::Block => children.map(s_).fold((0, 0), |(i_, o_), (i, o)| {
            (i_ + i.saturating_sub(o_), o + o_.saturating_sub(i))
        }),
        Op::Number(_) => unreachable!(),
        Op::Push => (0, 1),
        Op::Add
        | Op::Sub
        | Op::Mul
        | Op::Min
        | Op::Max
        | Op::Lt
        | Op::Le
        | Op::Eq
        | Op::Select
        | Op::Keep
        | Op::Join => (2, 1),
        Op::Fold => (assert_eq!(s_(children.next().unwrap()), (2, 1)), (2, 1)).1,
        Op::Scan => (assert_eq!(s_(children.next().unwrap()), (2, 1)), (1, 1)).1,
        Op::Length | Op::Iota | Op::Reverse | Op::Rise | Op::Fall | Op::Id => (1, 1),
        Op::Pop => (1, 0),
        Op::Fork => children.map(s_).fold((0, 0), fork),
        Op::Bracket => children.map(s_).fold((0, 0), bracket),
    }
}
