use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut map = HashMap::new();
    for _ in 0..n {
        input!(
            (a, c): (usize, Usize1),
        );
        let e = map.entry(c).or_insert(usize::MAX);
        if *e > a {
            *e = a;
        }
    }
    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
