use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );
    let mut ans = ModInt998244353::new(0);

    for i in 0..60_usize {
        if m >> i & 1 == 0 {
            continue;
        }
        let p: usize = 1 << (i + 1);
        let a = n / p;
        let b = n % p;
        ans += a * p / 2;
        if b >= p / 2 {
            ans += b - p / 2 + 1
        }
    }
    println!("{}", ans);
}
