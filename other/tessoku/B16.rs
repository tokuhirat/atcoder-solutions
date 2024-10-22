use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );

    let mut dp = vec![usize::MAX; n + 1];
    dp[1] = 0;
    for i in 1..n {
        let cost1 = dp[i] + (h[i]).abs_diff(h[i - 1]);
        if cost1 < dp[i + 1] {
            dp[i + 1] = cost1;
        }
        if i + 1 >= n {
            continue;
        }

        let cost2 = dp[i] + (h[i + 1]).abs_diff(h[i - 1]);
        if cost2 < dp[i + 2] {
            dp[i + 2] = cost2;
        }
    }
    println!("{}", dp[n]);
}
