use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::Chars};

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

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        s: Chars,
    };
    let mut st = Segtree::<M>::new(n + 1);
    for i in 0..n - 1 {
        if s[i] != s[i + 1] {
            st.set(i + 1, 1);
        }
    }

    for _ in 0..q {
        input! {(t, l, r): (u8, usize, usize)};
        if t == 1 {
            st.set(l - 1, 1 - st.get(l - 1));
            st.set(r, 1 - st.get(r));
        } else if st.prod(l..r) == r - l {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
