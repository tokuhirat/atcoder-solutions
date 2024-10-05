use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        h: usize,
        w: usize,
        c: [String; h]
    );
    let mut dp = vec![vec![0_usize; w]; h];
    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if c[i].chars().nth(j).unwrap() == '#' {
                continue;
            }
            if i < h - 1 {
                dp[i + 1][j] += dp[i][j];
            }
            if j < w - 1 {
                dp[i][j + 1] += dp[i][j];
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
