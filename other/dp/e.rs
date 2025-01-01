use std::mem::swap;

use proconio::{fastout, input};

const V: usize = 100_001;

#[fastout]
fn main() {
    input! {
        (n, w): (usize, usize),
    };

    let mut dp = vec![usize::MAX; V];
    dp[0] = 0;
    for _ in 0..n {
        input! {(wi, vi): (usize, usize)};
        let mut old = dp.clone();
        swap(&mut dp, &mut old);
        for (v_old, &w_old) in old
            .iter()
            .enumerate()
            .filter(|(_, &w_old)| w_old != usize::MAX)
        {
            let v_n = v_old + vi;
            let w_n = w_old + wi;
            if v_n < V {
                dp[v_n] = dp[v_n].min(w_n);
            }
        }
    }
    let ans = dp
        .iter()
        .enumerate()
        .filter(|(_, &wi)| wi <= w)
        .map(|(i, _)| i)
        .max()
        .unwrap();
    println!("{}", ans);
}
