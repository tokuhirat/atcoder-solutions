use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input!(
        n: usize,
        ac: [(usize, usize); n],
    );
    let mut a = Vec::with_capacity(n);
    let mut map = BTreeMap::new();
    for (i, &(ai, ci)) in ac.iter().enumerate() {
        a.push((ai, ci, i));
        map.insert(ci, i);
    }
    a.sort_unstable();
    let mut ans = Vec::new();
    for &(_, ci, i) in a.iter().take(n - 1) {
        map.remove(&ci);
        match map.range(..ci).next() {
            Some(_) => (),
            None => ans.push(i + 1),
        }
    }
    ans.push(a.last().unwrap().2 + 1);
    ans.sort_unstable();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
