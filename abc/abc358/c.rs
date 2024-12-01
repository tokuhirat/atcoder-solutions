use num::PrimInt;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );
    let mut p = Vec::with_capacity(n);
    for _ in 0..n {
        input!(s: Chars);
        let mut pi = 0;
        for (i, si) in s.iter().enumerate() {
            if si == &'o' {
                pi |= 1 << i;
            }
        }
        p.push(pi);
    }

    let mut ans = n;
    for i in 0..1 << n {
        let mut pi = 0;
        for (j, &pj) in p.iter().enumerate() {
            if i >> j & 1 != 0 {
                pi |= pj;
            }
        }
        if pi == (1 << m) - 1 {
            ans = ans.min(i.count_ones() as usize)
        }
    }
    println!("{}", ans);
}
