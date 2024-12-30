use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut ab = HashMap::new();
    for _ in 0..n {
        input! {(a, b): (usize, usize)};
        let (a, b) = (a.min(b), a.max(b));
        ab.insert(a, b);
    }

    let mut v = Vec::with_capacity(n);
    let mut s = HashSet::new();
    for i in 1..=2 * n {
        if let Some(&b) = ab.get(&i) {
            v.push(b);
            s.insert(b);
        }
        if s.contains(&i) {
            if let Some(j) = v.pop() {
                if j != i {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
