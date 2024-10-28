use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut ans = 0;
    for i in 0..15 {
        let num = n / 10_usize.pow(i) % 10;
        let c = n / 10_usize.pow(i + 1) * 10_usize.pow(i);
        for j in 0..=9 {
            match j.cmp(&num) {
                Ordering::Less => ans += (c + 10_usize.pow(i)) * j,
                Ordering::Equal => ans += (c + n % 10_usize.pow(i) + 1) * j,
                Ordering::Greater => ans += c * j,
            }
        }
    }
    println!("{}", ans);
}
