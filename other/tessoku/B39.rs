use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input!(
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    );

    xy.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut j = 0;
    let mut ans = 0;
    for di in 1..=d {
        while j < n && xy[j].0 <= di {
            heap.push(xy[j].1);
            j += 1
        }

        if let Some(job) = heap.pop() {
            ans += job;
        }
    }
    println!("{}", ans);
}
