use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    if n == 1 {
        println!("0");
        return;
    }
    let mut ans = vec![];
    let mut d = n - 1;
    while d > 0 {
        ans.push(d % 5);
        d /= 5;
    }
    let v = [0, 2, 4, 6, 8];
    println!("{}", ans.iter().rev().map(|&i| v[i]).join(""));
}
