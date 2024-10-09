use proconio::{fastout, input};
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input!(
        q: usize
    );

    let mut tree = BTreeSet::new();
    for _ in 0..q {
        input!(
            t: usize,
            x: i32
        );
        if t == 1 {
            tree.insert(x);
        } else if t == 2 {
            tree.remove(&x);
        } else {
            let mut ans = -1;
            for &ti in tree.iter() {
                if ti >= x {
                    ans = ti;
                    break;
                }
            }

            println!("{}", ans);
        }
    }
}
