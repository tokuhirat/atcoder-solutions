use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
    );
    let mut a = HashSet::new();
    for _ in 0..n {
        input!(ai: usize);
        if ai <= k {
            a.insert(ai);
        }
    }
    let mut ans = k * (k + 1) / 2;
    for ai in a.iter() {
        ans -= ai;
    }
    println!("{}", ans);
}
