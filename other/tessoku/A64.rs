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

    let mut adj = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        adj[a - 1].push((b - 1, c));
        adj[b - 1].push((a - 1, c));
    }

    let mut distance = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();
    let mut done = vec![false; n];
    distance[0] = 0;
    heap.push((Reverse(0), 0));
    while let Some((Reverse(d), current)) = heap.pop() {
        if done[current] {
            continue;
        }
        done[current] = true;

        for &(next_node, c) in adj[current].iter() {
            if d + c < distance[next_node] {
                distance[next_node] = d + c;
                heap.push((Reverse(distance[next_node]), next_node));
            }
        }
    }
    for &d in distance.iter() {
        if d == usize::MAX {
            println!("-1");
        } else {
            println!("{}", d);
        }
    }
}
