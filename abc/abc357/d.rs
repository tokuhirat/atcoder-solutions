use ac_library::ModInt998244353;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: u64,
    );
    let mut len: u64 = 1;
    while 10_usize.pow(len as u32) <= n as usize {
        len += 1;
    }
    let mut ans = ModInt998244353::new(n);
    let ten = ModInt998244353::new(10).pow(len);
    ans *= ten.pow(n) - 1;
    ans /= ten - 1;

    println!("{}", ans);
}
