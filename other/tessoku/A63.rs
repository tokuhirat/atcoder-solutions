use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let mut adj = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    let mut distance = vec![-1; n];
    distance[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        for &next_node in adj[current_node].iter() {
            if distance[next_node] == -1 {
                queue.push_back(next_node);
                distance[next_node] = distance[current_node] + 1;
            }
        }
    }
    for d in distance {
        println!("{}", d);
    }
}
