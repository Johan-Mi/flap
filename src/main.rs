fn main() {
    todo!();
}

enum Op {
    Push(Vec<i32>),
    Add,
    Sub,
    Mul,
    Select,
    Keep,
    Iota,
    Fold(Vec<Op>),
    Scan(Vec<Op>),
    Id,
    Fork(Vec<Vec<Op>>),
    Bracket(Vec<Vec<Op>>),
}

fn x(ops: &[Op], s: &mut Vec<Vec<i32>>) {
    for op in ops {
        x1(op, s);
    }
}

fn x1(op: &Op, s: &mut Vec<Vec<i32>>) {
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
            let f = |(a, b)| std::iter::repeat_n(b, usize::try_from(a).unwrap());
            s.push(std::iter::zip(a, b).flat_map(f).collect());
        }
        Op::Iota => {
            let [i] = g(s);
            let [i] = i.try_into().unwrap();
            s.push((0..i).collect());
        }
        Op::Fold(f) => {
            let [v] = g(s);
            let mut v: Vec<Vec<i32>> = v
                .into_iter()
                .map(|it| Vec::from([it]) as Vec<i32>)
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
                s.push(Vec::from([init]));
                for a in v {
                    s.push(Vec::from([a]));
                    x(f, s);
                    w.extend(s.last().unwrap());
                }
                let _: Vec<i32> = s.pop().unwrap();
            }
            s.push(w);
        }
        Op::Id => {}
        Op::Fork(f) => {
            let mut v = Vec::new();
            for f in f {
                let [i, _] = s_(f);
                v.extend(s[s.len() - i..].iter().cloned());
                x(f, &mut v);
            }
            s.truncate(s.len() - s1(op)[0]);
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

fn g<const N: usize>(s: &mut Vec<Vec<i32>>) -> [Vec<i32>; N] {
    std::array::from_fn(|_| s.pop().unwrap())
}

fn p2(s: &mut Vec<Vec<i32>>, f: fn(i32, i32) -> i32) {
    let [a, b] = g(s);
    let len = a.len().min(b.len());
    let ab = a.into_iter().cycle().zip(b.into_iter().cycle());
    s.push(ab.take(len).map(|(a, b)| f(a, b)).collect());
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
            [1, 1]
        }
        Op::Iota | Op::Id => [1, 1],
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
