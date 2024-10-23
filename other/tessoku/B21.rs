use std::cmp::max;

use proconio::{fastout, input, marker::Chars};

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars,
    );

    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i] = 1;
    }
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = 2;
        } else {
            dp[i][i + 1] = 1;
        }
    }

    for len in 2..n {
        for l in 0..n - len {
            let r = l + len;
            if s[l] == s[r] {
                dp[l][r] = max(dp[l + 1][r - 1] + 2, max(dp[l + 1][r], dp[l][r - 1]));
            } else {
                dp[l][r] = max(dp[l + 1][r], dp[l][r - 1]);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
}
