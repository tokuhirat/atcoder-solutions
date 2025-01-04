use proconio::{fastout, input, marker::Chars};

const MOD: usize = 1000000007;
#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        a: [Chars; h],
    };
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                continue;
            }
            if i > 0 && a[i - 1][j] == '.' {
                dp[i][j] += dp[i - 1][j];
                dp[i][j] %= MOD;
            }
            if j > 0 && a[i][j - 1] == '.' {
                dp[i][j] += dp[i][j - 1];
                dp[i][j] %= MOD;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
