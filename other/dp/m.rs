use ac_library::ModInt1000000007 as MInt;
use proconio::{fastout, input};
use std::mem::swap;

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    };
    let mut dp = vec![MInt::from(0); k + 1];
    dp[k] = 1.into();
    for &aj in &a {
        let mut old = vec![MInt::from(0); k + 1];
        swap(&mut dp, &mut old);

        for (i, &cnt) in old.iter().enumerate() {
            dp[i] += cnt;
            if i > aj {
                dp[i - aj - 1] -= cnt;
            }
        }

        dp = dp
            .iter()
            .rev()
            .scan(MInt::from(0), |state, &dpi| {
                *state += dpi;
                Some(*state)
            })
            .collect();
        dp.reverse();
    }
    println!("{}", dp[0]);
}
