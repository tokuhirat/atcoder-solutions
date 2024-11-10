use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        uvw: [(Usize1, Usize1, i64); m],
    );

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, -w));
    }
    let mut ans = vec![0_i64; n];
    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }
        ans[i] = 0;
        visited[i] = true;
        let mut queue = VecDeque::from([(i, 0_i64)]);
        while !queue.is_empty() {
            let (cur_u, cur_x) = queue.pop_front().unwrap();
            for &(nxt_v, nxt_w) in &graph[cur_u] {
                if !visited[nxt_v] {
                    visited[nxt_v] = true;
                    ans[nxt_v] = cur_x + nxt_w;
                    queue.push_back((nxt_v, ans[nxt_v]));
                }
            }
        }
    }
    for (i, &ai) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ai);
    }
    println!(" ");
}
