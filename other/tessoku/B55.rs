use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!(
        q: usize,
    );

    let mut set = BTreeSet::new();
    for _ in 0..q {
        input!(
            t: u8,
            x: i64,
        );
        if t == 1 {
            set.insert(x);
        } else {
            let mut diff = i64::MAX;
            if let Some(l) = set.range(..x).next_back() {
                diff = diff.min(x - l);
            }
            if let Some(u) = set.range(x..).next() {
                diff = diff.min(u - x);
            }

            if diff == i64::MAX {
                println!("-1");
            } else {
                println!("{}", diff);
            }
        }
    }
}
