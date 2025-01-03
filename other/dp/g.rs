use proconio::{fastout, input, marker::Usize1};

fn dfs(cur: usize, graph: &[Vec<usize>], len: &mut [i64]) -> i64 {
    if len[cur] != -1 {
        return len[cur];
    }
    let mut max = 0;
    for &nxt in &graph[cur] {
        max = max.max(dfs(nxt, graph, len) + 1);
    }
    len[cur] = max;
    len[cur]
}

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        xy: [(Usize1, Usize1); m],
    };
    let mut graph = vec![vec![]; n];
    for &(x, y) in &xy {
        graph[x].push(y);
    }
    let mut len = vec![-1; n];
    for i in 0..n {
        dfs(i, &graph, &mut len);
    }
    println!("{}", len.iter().max().unwrap());
}
