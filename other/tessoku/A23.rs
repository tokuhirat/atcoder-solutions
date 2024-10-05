use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        a: [[usize; n]; m]
    );

    let inf = 1 << 20;

    let mut dp = vec![vec![inf; 1 << n]; m + 1];
    dp[0][0] = 0;
    for (i, ai) in a.iter().enumerate() {
        let mut mi = 0;
        for (j, aij) in ai.iter().enumerate() {
            if aij != &0 {
                mi |= 1 << j;
            }
        }

        for j in 0..1 << n {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            dp[i + 1][j | mi] = dp[i + 1][j | mi].min(dp[i][j] + 1);
        }
    }
    if dp[m][(1 << n) - 1] < inf {
        println!("{}", dp[m][(1 << n) - 1]);
    } else {
        println!("-1");
    }
}
