use ac_library::{LazySegtree, MapMonoid, Monoid};
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
        x: [usize; m],
    };
    let mut sg = LazySegtree::<F>::new(n);
    let mut ans = 0;
    for i in 0..m - 1 {
        let s = x[i].min(x[i + 1]);
        let t = x[i].max(x[i + 1]);
        let a = t - s;
        let b = n - a;
        match a.cmp(&b) {
            std::cmp::Ordering::Greater => {
                ans += b;
                sg.apply_range(0..s, a - b);
                sg.apply_range(t..n, a - b);
            }
            std::cmp::Ordering::Less => {
                ans += a;
                sg.apply_range(s..t, b - a);
            }
            std::cmp::Ordering::Equal => {
                ans += a;
            }
        }
    }
    ans += (0..n).map(|e| sg.get(e)).min().unwrap();
    println!("{}", ans);
}
