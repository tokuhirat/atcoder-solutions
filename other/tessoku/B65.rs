use proconio::{fastout, input, marker::Usize1};

fn dfs(graph: &[Vec<usize>], from: usize, visited: &mut [bool], rank: &mut [usize]) -> usize {
    let mut rank_max = 0;

    let mut is_leaf = true;
    for &to in &graph[from] {
        if visited[to] {
            continue;
        }
        is_leaf = false;
        visited[to] = true;
        let r = dfs(graph, to, visited, rank);
        if r > rank_max {
            rank_max = r;
        }
    }
    if is_leaf {
        return 0;
    }
    rank_max += 1;
    rank[from] = rank_max;
    rank_max
}

#[fastout]
fn main() {
    input!(
        (n, t): (usize, Usize1),
        ab: [(Usize1, Usize1); n - 1],
    );

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut visited = vec![false; n];
    visited[t] = true;
    let mut rank = vec![0; n];
    dfs(&graph, t, &mut visited, &mut rank);

    for (i, &r) in rank.iter().enumerate() {
        if i > 0 {
            print!(" ")
        }
        print!("{}", r);
    }
    println!(" ");
}
