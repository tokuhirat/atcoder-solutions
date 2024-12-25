use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::Usize1};
use std::{cmp::Reverse, collections::BTreeMap};

fn add(map: &mut BTreeMap<Reverse<usize>, usize>, v: usize, num: usize) {
    let e = map.entry(Reverse(v)).or_insert(0);
    *e += num;
}

struct M;
impl Monoid for M {
    type S = (usize, usize, usize, usize);
    fn identity() -> Self::S {
        (0, 0, 0, 0)
    }

    fn binary_operation(&(a1, n1, a2, n2): &Self::S, &(b1, m1, b2, m2): &Self::S) -> Self::S {
        let mut map = BTreeMap::new();
        add(&mut map, 0, 0);
        add(&mut map, a1, n1);
        add(&mut map, a2, n2);
        add(&mut map, b1, m1);
        add(&mut map, b2, m2);
        let (Reverse(p), n) = map.pop_first().unwrap();
        if let Some((Reverse(q), m)) = map.pop_first() {
            (p, n, q, m)
        } else {
            (p, n, 0, 0)
        }
    }
}

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        a: [usize; n],
    );

    let mut tree: Segtree<M> = a.iter().map(|&e| (e, 1, 0, 0)).collect::<Vec<_>>().into();
    for _ in 0..q {
        input!(t: usize);
        if t == 1 {
            input!((p, x): (Usize1, usize));
            tree.set(p, (x, 1, 0, 0));
        } else {
            input!((l, r): (Usize1, usize));
            let (_, _, _, m) = tree.prod(l..r);
            println!("{}", m);
        }
    }
}
