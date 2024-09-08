use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: usize,
        a: [usize; n]
    );

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                dp[i][j] = dp[i - 1][j];
            } else if dp[i - 1][j] || dp[i - 1][j - a[i - 1]] {
                dp[i][j] = true;
            }
        }
    }

    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
