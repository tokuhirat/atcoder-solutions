use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );
    let mut dp = vec![vec![0; n]; n];
    for (i, &ai) in a.iter().enumerate() {
        dp[n - 1][i] = ai;
    }
    for i in (1..=n - 1).rev() {
        for j in 0..i {
            if i % 2 == 1 {
                dp[i - 1][j] = dp[i][j].max(dp[i][j + 1]);
            } else {
                dp[i - 1][j] = dp[i][j].min(dp[i][j + 1]);
            }
        }
    }
    println!("{}", dp[0][0]);
}
