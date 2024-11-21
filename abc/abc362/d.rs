use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        a: [usize; n],
    );
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input!(
            (u, v, b): (Usize1, Usize1, usize),
        );
        graph[u].push((v, b));
        graph[v].push((u, b));
    }
    let mut dist = vec![usize::MAX; n];
    dist[0] = a[0];
    let mut queue = BinaryHeap::from([(Reverse(a[0]), 0)]);
    while let Some((Reverse(d), cur)) = queue.pop() {
        if d > dist[cur] {
            continue;
        }
        for &(nxt, nxt_d) in &graph[cur] {
            if d + nxt_d + a[nxt] < dist[nxt] {
                dist[nxt] = d + nxt_d + a[nxt];
                queue.push((Reverse(dist[nxt]), nxt));
            }
        }
    }
    println!("{}", dist.iter().skip(1).join(" "));
}
