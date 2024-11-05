use proconio::{fastout, input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    );

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut distance = vec![(usize::MAX, usize::MAX); n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(cost), cur)) = heap.pop() {
        for &(nxt, d) in &graph[cur] {
            if cost + d < distance[nxt].1 {
                distance[nxt] = (cur, cost + d);
                heap.push((Reverse(distance[nxt].1), nxt));
            }
        }
    }

    let mut ans = vec![n - 1; 1];
    let mut cur = n - 1;
    while cur != 0 {
        ans.push(distance[cur].0);
        cur = distance[cur].0;
    }

    for (i, a) in ans.iter().rev().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a + 1);
    }
    println!(" ");
}
