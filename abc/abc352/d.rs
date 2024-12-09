use proconio::{fastout, input, marker::Usize1};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        p: [Usize1; n],
    );
    let mut idx = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        idx[pi] = i;
    }
    let mut set = BTreeSet::new();
    for &i in idx.iter().take(k) {
        set.insert(i);
    }
    let mut ans = set.last().unwrap() - set.first().unwrap();
    for i in 0..n - k {
        set.remove(&idx[i]);
        set.insert(idx[i + k]);
        ans = ans.min(set.last().unwrap() - set.first().unwrap());
    }
    println!("{}", ans);
}
