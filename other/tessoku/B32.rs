use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        a: [usize; k]
    );

    let mut dp = vec![false; n + 1];
    for j in 1..=n {
        for &ai in a.iter() {
            if j >= ai && !dp[j - ai] {
                dp[j] = true;
                break;
            }
        }
    }
    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
