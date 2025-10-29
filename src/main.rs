fn main() {
    todo!();
}

enum Op {
    Add,
    Sub,
    Mul,
    Select,
    Keep,
    Fold(Box<[Op]>),
    Scan(Box<[Op]>),
    Fill([Box<[Op]>; 2]),
    Id,
    Fork(Box<[Box<[Op]>]>),
    Bracket(Box<[Box<[Op]>]>),
}

fn x(ops: &[Op], s: &mut Vec<Box<[i32]>>, fill: Option<&[Op]>) {
    for op in ops {
        x1(op, s, fill);
    }
}

fn x1(op: &Op, s: &mut Vec<Box<[i32]>>, fill: Option<&[Op]>) {
    match op {
        Op::Add => p2(s, std::ops::Add::add),
        Op::Sub => p2(s, std::ops::Sub::sub),
        Op::Mul => p2(s, std::ops::Mul::mul),
        Op::Select => {
            let [a, b] = g(s);
            s.push(b.iter().map(|&i| a[usize::try_from(i).unwrap()]).collect());
        }
        Op::Keep => {
            let [a, b] = g(s);
            assert_eq!(a.len(), b.len());
            let k = std::iter::zip(a, b).filter_map(|(a, b)| (b != 0).then_some(a));
            s.push(k.collect());
        }
        Op::Fold(f) => {
            let [v] = g(s);
            x(fill.unwrap(), s, None);
            for a in v {
                s.push(Box::new([a]));
                x(f, s, None);
            }
        }
        Op::Scan(_) => todo!(),
        Op::Fill([fill, b]) => x(b, s, Some(fill)),
        Op::Id => {}
        Op::Fork(_) => todo!(),
        Op::Bracket(_) => todo!(),
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

fn s(ops: &[Op]) -> [usize; 2] {
    ops.iter().map(s1).fold([0, 0], |[i_, o_], [i, o]| {
        [i_ + i.saturating_sub(o_), o + o_.saturating_sub(i)]
    })
}

fn s1(op: &Op) -> [usize; 2] {
    match op {
        Op::Add | Op::Sub | Op::Mul | Op::Select | Op::Keep => [2, 1],
        Op::Fold(v) | Op::Scan(v) => {
            assert!(s(v) == [2, 1]);
            [1, 1]
        }
        Op::Fill([f, v]) => {
            assert!(s(f) == [0, 1]);
            s(v)
        }
        Op::Id => [1, 1],
        Op::Fork(vs) => vs
            .iter()
            .map(|v| s(v))
            .fold([0, 0], |[i1, o1], [i2, o2]| [i1.max(i2), o1 + o2]),
        Op::Bracket(vs) => vs
            .iter()
            .map(|v| s(v))
            .fold([0, 0], |[i1, o1], [i2, o2]| [i1 + i2, o1 + o2]),
    }
}
