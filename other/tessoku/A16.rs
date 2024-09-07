use std::cmp::min;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 2]
    );

    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = min(dp[i - 2] + b[i - 2], dp[i - 1] + a[i - 1]);
    }
    println!("{}", dp[n - 1]);
}
