use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    );

    let max_n = 100;
    let max_v = 1000;
    let max_value = max_n * max_v;
    let mut dp = vec![vec![usize::MAX / 2; max_value + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for value in 0..=max_value {
            dp[i][value] = dp[i - 1][value];
            if value >= vi {
                dp[i][value] = dp[i][value].min(dp[i - 1][value - vi] + wi);
            }
        }
    }

    let mut ans = 0;
    for v in 0..=max_value {
        if dp[n][v] <= w {
            ans = v;
        }
    }
    println!("{}", ans);
}
