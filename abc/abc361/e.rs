use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

fn bfs(graph: &[Vec<(usize, i64)>], start: usize) -> (usize, i64) {
    let n = graph.len();
    let mut dist: Vec<i64> = vec![-1; n];
    let mut queue = VecDeque::new();
    dist[start] = 0;
    queue.push_back(start);
    while let Some(cur) = queue.pop_front() {
        for &(nxt, d) in &graph[cur] {
            if dist[nxt] == -1 {
                dist[nxt] = dist[cur] + d;
                queue.push_back(nxt);
            }
        }
    }
    let mut max = 0;
    let mut idx = 0;
    for (i, &d) in dist.iter().enumerate() {
        if d > max {
            max = d;
            idx = i;
        }
    }
    (idx, max)
}

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut ans = 0;
    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input!((a, b, c): (Usize1, Usize1, i64));
        graph[a].push((b, c));
        graph[b].push((a, c));
        ans += 2 * c;
    }
    let (idx, _) = bfs(&graph, 0);
    let (_, d) = bfs(&graph, idx);
    ans -= d;
    println!("{}", ans);
}
