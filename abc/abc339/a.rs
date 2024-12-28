use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };
    let mut ans = vec![];
    for &si in s.iter().rev() {
        if si == '.' {
            break;
        }
        ans.push(si);
    }
    println!("{}", ans.iter().rev().join(""));
}
