use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        s: String, t: String,
    );

    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for (i, si) in s.iter().enumerate() {
        for (j, tj) in t.iter().enumerate() {
            if si == tj {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
