use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::{fastout, input, marker::Usize1};

struct M;
impl Monoid for M {
    type S = usize;
    fn identity() -> Self::S {
        usize::MAX
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.min(b)
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = usize;

    fn identity_map() -> Self::F {
        usize::MAX
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        *f.min(x)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        *f.min(g)
    }
}

#[fastout]
fn main() {
    input!(
        (h, w, n): (usize, usize, usize),
    );
    let mut rcl = Vec::with_capacity(n);
    for i in 0..n {
        input!((r, c, l): (usize, Usize1, usize));
        rcl.push((r, c, l, i));
    }
    rcl.sort_unstable();
    rcl.reverse();

    let mut segtree = LazySegtree::<F>::new(w);
    for i in 0..w {
        segtree.set(i, h);
    }

    let mut ans = vec![0; n];
    for &(_, c, l, i) in &rcl {
        let y = segtree.prod(c..c + l);
        ans[i] = y;
        segtree.apply_range(c..c + l, y - 1);
    }
    for a in ans {
        println!("{}", a);
    }
}
