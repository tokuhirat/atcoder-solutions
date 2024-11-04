use proconio::{fastout, input};

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
    let mut max_num = 0;
    let mut ans = 0;
    for (i, p) in graph.iter().enumerate() {
        if max_num < p.len() {
            ans = i;
            max_num = p.len();
        }
    }
    println!("{}", ans + 1);
}
