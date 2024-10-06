use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: usize,
        b: usize,
    );

    let mut dp = vec![false; n + 1];
    for i in 0..=n {
        dp[i] = (i >= a && !dp[i - a]) || (i >= b && !dp[i - b]);
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
