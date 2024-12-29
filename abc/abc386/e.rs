use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    };
    let mut ans = 0;
    if 2 * k < n {
        for p in a.iter().combinations(k) {
            ans = ans.max(p.iter().map(|&&e| e).fold(0, |acc, e| acc ^ e));
        }
    } else {
        let b = a.iter().fold(0, |acc, e| acc ^ e);
        for p in a.iter().combinations(n - k) {
            ans = ans.max(b ^ (p.iter().map(|&&e| e).fold(0, |acc, e| acc ^ e)));
        }
    }
    println!("{}", ans);
}
