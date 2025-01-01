use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, w): (usize, usize),
    };
    let mut dp = vec![i64::MIN; w + 1];
    dp[0] = 0;
    for _ in 0..n {
        input! {(wi, vi): (usize, i64)};
        let mut old = dp.clone();
        swap(&mut dp, &mut old);
        for (w_c, &v_c) in old.iter().enumerate().filter(|(_, &v_c)| v_c >= 0) {
            let w_n = w_c + wi;
            let v_n = v_c + vi;
            if w_n <= w {
                dp[w_n] = dp[w_n].max(v_n);
            }
        }
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
