use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    );

    let mut adj = vec![vec![]; n];
    let mut visited = vec![0; n];

    for (a, b) in ab.iter() {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    dfs(&adj, 0, &mut visited);
    for &v in visited.iter() {
        if v != 1 {
            println!("The graph is not connected.");
            return;
        }
    }
    println!("The graph is connected.")
}

fn dfs(adj: &Vec<Vec<usize>>, current: usize, visited: &mut [usize]) {
    visited[current] = 1;
    for &next_node in adj[current].iter() {
        if visited[next_node] == 0 {
            dfs(adj, next_node, visited)
        }
    }
}
