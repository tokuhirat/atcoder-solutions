use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 1],
    );

    let mut dp = vec![0; n];
    for i in 0..n - 1 {
        let mut score_a = dp[i] + 100;
        if i != 0 && dp[i] == 0 {
            score_a = 0;
        }
        if dp[a[i] - 1] < score_a {
            dp[a[i] - 1] = score_a;
        }

        let mut score_b = dp[i] + 150;
        if i != 0 && dp[i] == 0 {
            score_b = 0;
        }

        if dp[b[i] - 1] < score_b {
            dp[b[i] - 1] = score_b;
        }
    }
    println!("{}", dp[n - 1]);
}
