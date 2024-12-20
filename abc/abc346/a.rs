use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );
    let mut ans = Vec::with_capacity(n);
    for i in 0..n - 1 {
        ans.push(a[i] * a[i + 1]);
    }
    println!("{}", ans.iter().join(" "));
}
