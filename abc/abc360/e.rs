use ac_library::ModInt998244353 as ModInt;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
    );

    let inv = ModInt::new(n * n).inv();
    let mut p = ModInt::new(1);
    for _ in 0..k {
        p = p * (n * n - 2 * n + 2) + (ModInt::new(1) - p) * 2;
        p *= inv;
    }
    let ans = p + (ModInt::new(1) - p) * (n + 2) / 2;
    println!("{}", ans);
}
