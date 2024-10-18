use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    );

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        graph[a - 1].push((Reverse(c), b - 1));
        graph[b - 1].push((Reverse(c), a - 1));
    }

    let mut heap = BinaryHeap::from(graph[0].clone());
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut ans = 0;

    while let Some((Reverse(cost), pos)) = heap.pop() {
        if seen[pos] {
            continue;
        }
        seen[pos] = true;
        ans += cost;
        for &(next_cost, next_pos) in graph[pos].iter() {
            if !seen[next_pos] {
                heap.push((next_cost, next_pos));
            }
        }
    }
    println!("{}", ans);
}
