use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::{fastout, input};

struct M;
impl Monoid for M {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize; n],
        b: [usize; m],
    };
    let mut st = LazySegtree::<F>::new(n);
    for (i, &ai) in a.iter().enumerate() {
        st.set(i, ai);
    }
    for &bi in &b {
        let ball = st.get(bi);
        let num = ball / n;
        let res = ball % n;
        st.set(bi, 0);
        st.apply_range(0..n, num);
        if bi + res < n {
            st.apply_range(bi + 1..=bi + res, 1);
        } else {
            st.apply_range(bi + 1..n, 1);
            st.apply_range(0..=(bi + res) % n, 1);
        }
    }

    println!("{}", (0..n).map(|i| st.get(i)).join(" "));
}
