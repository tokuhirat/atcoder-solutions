use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );

    let mut c = vec![BTreeSet::new(); 3001];
    for (i, &hi) in h.iter().enumerate() {
        c[hi].insert(i);
    }

    let mut ans = 1;
    for ci in &c {
        if ci.is_empty() {
            continue;
        }
        for (i, &l) in ci.iter().enumerate() {
            for &r in ci.iter().skip(i + 1) {
                let d = r - l;
                let mut a = r;
                while ci.contains(&a) {
                    a += d;
                }
                ans = ans.max((a - l) / d);
            }
        }
    }
    println!("{}", ans);
}
