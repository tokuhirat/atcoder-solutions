use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        s: String,
    );
    let n = s.len();
    let mut p = HashSet::new();
    for i in 0..n {
        for j in i + 1..=n {
            p.insert(&s[i..j]);
        }
    }
    println!("{}", p.len());
}
