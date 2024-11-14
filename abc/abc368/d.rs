use proconio::{fastout, input, marker::Usize1};

fn dfs(
    tree: &[Vec<usize>],
    visited: &mut [bool],
    cur: usize,
    nodes: &[usize],
    ans: &mut [bool],
) -> bool {
    visited[cur] = true;
    let mut contain = false;
    for &nxt in &tree[cur] {
        if !visited[nxt] {
            contain |= dfs(tree, visited, nxt, nodes, ans);
        }
    }
    if nodes.binary_search(&cur).is_ok() | contain {
        ans[cur] = true;
    }
    ans[cur]
}

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        ab: [(Usize1, Usize1); n - 1],
        v: [Usize1; k]
    );
    let mut tree = vec![vec![]; n];
    for &(a, b) in &ab {
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut ans = vec![false; n];
    let mut visited = vec![false; n];
    dfs(&tree, &mut visited, v[0], &v, &mut ans);
    let ans = ans.iter().fold(0, |acc, &e| acc + if e { 1 } else { 0 });
    println!("{}", ans);
}
