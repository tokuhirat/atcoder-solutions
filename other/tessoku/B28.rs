use proconio::{fastout, input};

const MOD_N: usize = 1000000007;

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut a = 1;
    let mut b = 1;
    for _ in 0..n - 2 {
        (a, b) = (b, (a + b) % MOD_N);
    }
    println!("{}", b);
}
