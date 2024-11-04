use proconio::{fastout, input};

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, path: &mut Vec<usize>, n: usize) -> bool {
    let cur = *path.last().unwrap();
    if cur == n - 1 {
        return true;
    }
    visited[cur] = true;
    for &nxt in graph[cur].iter() {
        if visited[nxt] {
            continue;
        }
        path.push(nxt);
        if dfs(graph, visited, path, n) {
            return true;
        }
        path.pop();
    }
    false
}

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
        graph[b - 1].push(a - 1);
    }
    let mut visited = vec![false; n];
    let mut path = vec![0; 1];
    dfs(&graph, &mut visited, &mut path, n);
    for (i, &p) in path.iter().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", p + 1);
    }
    println!("");
}
