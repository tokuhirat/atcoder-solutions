use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        (n, a, b): (usize, usize, usize),
        d: [usize; n],
    );
    let mut set = HashSet::new();
    for &di in &d {
        set.insert(di % (a + b));
    }
    let mut d_r: Vec<usize> = set.into_iter().collect();
    d_r.sort_unstable();
    d_r.push(d_r.first().unwrap() + a + b);
    for i in 0..d_r.len() - 1 {
        if d_r[i + 1] - d_r[i] > b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
