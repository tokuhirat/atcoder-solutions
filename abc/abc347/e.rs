use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        x: [Usize1; q],
    );
    let mut s = HashSet::new();
    let mut size = Vec::with_capacity(q + 1);
    size.push(0);
    let mut t = vec![vec![]; n];
    for (i, &xi) in x.iter().enumerate() {
        t[xi].push(i);
        if s.contains(&xi) {
            s.remove(&xi);
        } else {
            s.insert(xi);
        }
        size.push(size.last().unwrap() + s.len());
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        if t[i].len() % 2 == 1 {
            t[i].push(q);
        }
        for j in 0..t[i].len() / 2 {
            let l = t[i][2 * j];
            let r = t[i][2 * j + 1];
            ans[i] += size[r] - size[l];
        }
    }
    println!("{}", ans.iter().join(" "));
}
