use itertools::Itertools;
use proconio::{fastout, input};

fn dfs(n: usize, m: usize, v: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if v.len() == n {
        result.push(v.clone());
        return;
    }
    let start = if v.is_empty() {
        1
    } else {
        v.last().unwrap() + 10
    };
    let end = m - (n - v.len() - 1) * 10;
    for i in start..=end {
        v.push(i);
        dfs(n, m, v, result);
        v.pop();
    }
}

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );
    let mut v = vec![];
    let mut result = vec![];
    dfs(n, m, &mut v, &mut result);
    println!("{}", result.len());
    for r in result {
        println!("{}", r.iter().join(" "));
    }
}
