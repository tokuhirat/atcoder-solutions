use proconio::{fastout, input};

const MOD_N: usize = 1000000007;

fn powmod(base: usize, exp: usize) -> usize {
    let mut ans = 1;
    let mut p = base;
    for i in 0..60 {
        if i != 0 {
            p *= p;
            p %= MOD_N;
        }
        if exp & (1 << i) != 0 {
            ans *= p;
            ans %= MOD_N;
        }
    }
    ans
}

#[fastout]
fn main() {
    input!(
        a: usize,
        b: usize,
    );
    println!("{}", powmod(a, b));
}
