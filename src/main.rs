fn main() {
    todo!();
}

enum Op {
    Push(Box<[i32]>),
    Add,
    Sub,
    Mul,
    Select,
    Keep,
    Fold(Box<[Op]>),
    Scan(Box<[Op]>),
    Id,
    Fork(Box<[Box<[Op]>]>),
    Bracket(Box<[Box<[Op]>]>),
}

fn x(ops: &[Op], s: &mut Vec<Box<[i32]>>) {
    for op in ops {
        x1(op, s);
    }
}

fn x1(op: &Op, s: &mut Vec<Box<[i32]>>) {
    match op {
        Op::Push(v) => s.push(v.clone()),
        Op::Add => p2(s, std::ops::Add::add),
        Op::Sub => p2(s, std::ops::Sub::sub),
        Op::Mul => p2(s, std::ops::Mul::mul),
        Op::Select => {
            let [a, b] = g(s);
            s.push(a.iter().map(|&i| b[usize::try_from(i).unwrap()]).collect());
        }
        Op::Keep => {
            let [a, b] = g(s);
            assert_eq!(a.len(), b.len());
            let k = std::iter::zip(a, b).filter_map(|(a, b)| (a != 0).then_some(b));
            s.push(k.collect());
        }
        Op::Fold(f) => {
            let [v] = g(s);
            let mut v: Vec<Box<[i32]>> = v
                .into_iter()
                .map(|it| Box::new([it]) as Box<[i32]>)
                .collect();
            for _ in 0..v.len().checked_sub(1).unwrap() {
                x(f, &mut v);
            }
            s.push(v.pop().unwrap());
        }
        Op::Scan(f) => {
            let [v] = g(s);
            let mut w = Vec::with_capacity(v.len());
            let mut v = v.into_iter();
            if let Some(init) = v.next() {
                w.push(init);
                s.push(Box::new([init]));
                for a in v {
                    s.push(Box::new([a]));
                    x(f, s);
                    w.push(s.last().unwrap()[0]);
                }
                let _: Box<[i32]> = s.pop().unwrap();
            }
            s.push(w.into());
        }
        Op::Id => {}
        Op::Fork(f) => {
            let mut v = Vec::new();
            for f in f {
                let [i, _] = s_(f);
                v.extend(s[s.len() - i..].iter().cloned());
                x(f, s);
            }
            s.extend(v);
        }
        Op::Bracket(f) => {
            let mut v = Vec::new();
            for f in f {
                let [_, o] = s_(f);
                x(f, s);
                v.extend(s.drain(s.len() - o..));
            }
            s.extend(v);
        }
    }
}

fn g<const N: usize>(s: &mut Vec<Box<[i32]>>) -> [Box<[i32]>; N] {
    std::array::from_fn(|_| s.pop().unwrap())
}

fn p2(s: &mut Vec<Box<[i32]>>, f: fn(i32, i32) -> i32) {
    let [a, b] = g(s);
    assert_eq!(a.len(), b.len());
    s.push(std::iter::zip(a, b).map(|(a, b)| f(a, b)).collect());
}

fn s_(ops: &[Op]) -> [usize; 2] {
    ops.iter().map(s1).fold([0, 0], |[i_, o_], [i, o]| {
        [i_ + i.saturating_sub(o_), o + o_.saturating_sub(i)]
    })
}

fn s1(op: &Op) -> [usize; 2] {
    match op {
        Op::Push(_) => [0, 1],
        Op::Add | Op::Sub | Op::Mul | Op::Select | Op::Keep => [2, 1],
        Op::Fold(v) | Op::Scan(v) => {
            assert!(s_(v) == [2, 1]);
            [2, 1]
        }
        Op::Id => [1, 1],
        Op::Fork(vs) => vs
            .iter()
            .map(|v| s_(v))
            .fold([0, 0], |[i1, o1], [i2, o2]| [i1.max(i2), o1 + o2]),
        Op::Bracket(vs) => vs
            .iter()
            .map(|v| s_(v))
            .fold([0, 0], |[i1, o1], [i2, o2]| [i1 + i2, o1 + o2]),
    }
}
