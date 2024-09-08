use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize, w: usize,
        wv: [(usize, usize); n]
    );

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for j in 0..=w {
            if j < wi {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - wi] + vi);
            }
        }
    }
    println!("{}", dp[n][w])
}
