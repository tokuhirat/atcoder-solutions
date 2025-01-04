use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [f64; n],
    };
    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0;
    for &pi in p.iter() {
        let mut old = vec![0.0; n + 1];
        swap(&mut dp, &mut old);
        for (j, &qj) in old.iter().enumerate() {
            dp[j] = qj * (1.0 - pi);
            if j > 0 {
                dp[j] += old[j - 1] * pi;
            }
        }
    }
    let ans: f64 = dp.iter().skip((n + 1) / 2).sum();
    println!("{}", ans);
}
