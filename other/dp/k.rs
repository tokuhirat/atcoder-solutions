use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    };
    let mut dp = vec![false; k + 1];

    for i in 1..dp.len() {
        dp[i] = !a.iter().filter(|&&ai| ai <= i).all(|ai| dp[i - ai]);
    }
    if dp[k] {
        println!("First");
    } else {
        println!("Second");
    }
}
