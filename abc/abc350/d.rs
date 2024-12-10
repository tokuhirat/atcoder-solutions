use proconio::{fastout, input, marker::Usize1};

fn dfs(graph: &[Vec<usize>], seen: &mut [bool], i: usize) -> usize {
    seen[i] = true;
    let mut ret = 1;
    for &nxt in &graph[i] {
        if seen[nxt] {
            continue;
        }
        ret += dfs(graph, seen, nxt);
    }
    ret
}

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    );
    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let c = dfs(&graph, &mut seen, i);
        ans += c * c.saturating_sub(1) / 2;
    }
    ans -= m;
    println!("{}", ans);
}
