use proconio::{fastout, input};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut is_even = n % 2 == 0;
    let mut dp: Vec<_> = a.iter().map(|&ai| if is_even { -ai } else { ai }).collect();
    let mut len = 1;
    while len < n {
        is_even = !is_even;
        let mut dpn = vec![if is_even { i64::MAX / 2 } else { i64::MIN / 2 }; n - len];
        for i in 0..n - len {
            if is_even {
                dpn[i] = dpn[i].min(dp[i] - a[i + len]);
                dpn[i] = dpn[i].min(dp[i + 1] - a[i]);
            } else {
                dpn[i] = dpn[i].max(dp[i] + a[i + len]);
                dpn[i] = dpn[i].max(dp[i + 1] + a[i]);
            }
        }
        swap(&mut dp, &mut dpn);
        len += 1;
    }
    println!("{}", dp[0]);
}
