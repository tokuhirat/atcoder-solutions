use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let mut graph = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        graph[a - 1].push(b - 1);
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![usize::MAX; n];
    queue.push_back(0);
    dist[0] = 0;

    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        for &nxt in graph[cur].iter() {
            if nxt == 0 {
                println!("{}", dist[cur] + 1);
                return;
            }
            if dist[nxt] != usize::MAX {
                continue;
            }
            dist[nxt] = dist[cur] + 1;
            queue.push_back(nxt);
        }
    }
    println!("-1");
}
