use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut dp = [0, 0, 0];
    for i in 0..n {
        input! {(a, b, c): (usize, usize, usize)};
        if i == 0 {
            dp = [a, b, c];
        } else {
            let mut old = [0, 0, 0];
            swap(&mut dp, &mut old);
            dp[0] = old[1].max(old[2]) + a;
            dp[1] = old[2].max(old[0]) + b;
            dp[2] = old[0].max(old[1]) + c;
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
