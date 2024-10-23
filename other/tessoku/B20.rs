use std::cmp::min;

use proconio::{fastout, input, marker::Chars};

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input!(
        s: Chars,
        t: Chars,
    );

    let mut dp = vec![vec![usize::MAX; s.len() + 1]; t.len() + 1];

    for i in 0..=t.len() {
        dp[i][0] = i;
    }
    for i in 0..=s.len() {
        dp[0][i] = i;
    }

    for i in 1..=t.len() {
        for j in 1..=s.len() {
            if s[j - 1] == t[i - 1] {
                dp[i][j] = min(dp[i - 1][j - 1], min(dp[i][j - 1] + 1, dp[i - 1][j] + 1));
            } else {
                dp[i][j] = min(
                    dp[i - 1][j - 1] + 1,
                    min(dp[i][j - 1] + 1, dp[i - 1][j] + 1),
                );
            }
        }
    }
    println!("{}", dp[t.len()][s.len()]);
}
