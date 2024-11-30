use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (_n, d): (usize, usize),
        mut s: Chars,
    );
    for _ in 0..d {
        for si in s.iter_mut().rev() {
            if *si == '@' {
                *si = '.';
                break;
            }
        }
    }
    println!("{}", s.iter().join(""));
}
