use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let mut out = HashSet::new();

    for &(ai, bi) in ab.iter() {
        out.insert((ai, bi));
        // (i+2,j+1)
        if ai + 2 <= n && bi + 1 <= n {
            out.insert((ai + 2, bi + 1));
        }
        // (i+1,j+2)
        if ai + 1 <= n && bi + 2 <= n {
            out.insert((ai + 1, bi + 2));
        }
        // (i−1,j+2)
        if ai >= 2 && bi + 2 <= n {
            out.insert((ai - 1, bi + 2));
        }
        // (i−2,j+1)
        if ai >= 3 && bi + 1 <= n {
            out.insert((ai - 2, bi + 1));
        }
        // (i−2,j−1)
        if ai >= 3 && bi >= 2 {
            out.insert((ai - 2, bi - 1));
        }
        // (i−1,j−2)
        if ai >= 2 && bi >= 3 {
            out.insert((ai - 1, bi - 2));
        }
        // (i+1,j−2)
        if ai + 1 <= n && bi >= 3 {
            out.insert((ai + 1, bi - 2));
        }
        // (i+2,j−1)
        if ai + 2 <= n && bi >= 2 {
            out.insert((ai + 2, bi - 1));
        }
    }
    let ans = n * n - out.len();
    println!("{}", ans);
}
