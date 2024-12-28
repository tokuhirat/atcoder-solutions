use proconio::{fastout, input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut edges = vec![vec![]; n];
    for i in 0..n - 1 {
        input! {(a, b, x): (usize, usize, Usize1)};
        edges[i + 1].push((i, a));
        edges[x].push((i, b));
    }

    let mut time = vec![usize::MAX; n];
    time[n - 1] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), n - 1));
    while let Some((Reverse(t), i)) = heap.pop() {
        if time[i] != t {
            continue;
        }
        for &(ni, nt) in &edges[i] {
            if time[ni] > t + nt {
                heap.push((Reverse(t + nt), ni));
                time[ni] = t + nt;
            }
        }
    }
    println!("{}", time[0]);
}
