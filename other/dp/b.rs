use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        h: [usize; n],
    };
    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k {
            let nxt = i + j;
            if nxt < n {
                dp[nxt] = dp[nxt].min(dp[i] + h[i].abs_diff(h[nxt]));
            }
        }
    }
    println!("{}", dp[n - 1]);
}
