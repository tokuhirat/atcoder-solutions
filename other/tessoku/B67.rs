use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;

fn root(uf: &mut [usize], x: usize) -> usize {
    if uf[x] == x {
        return x;
    }
    uf[x] = root(uf, uf[x]);
    uf[x]
}

fn merge(uf: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
    let mut root_a = root(uf, a);
    let mut root_b = root(uf, b);
    if root_a == root_b {
        return;
    }

    if rank[root_a] < rank[root_b] {
        (root_a, root_b) = (root_b, root_a);
    }
    uf[root_b] = root_a;
    if rank[root_a] == rank[root_b] {
        rank[root_a] += 1;
    }
}

fn is_same(uf: &mut [usize], a: usize, b: usize) -> bool {
    root(uf, a) == root(uf, b)
}

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    );

    let mut edge = vec![];
    for &(a, b, c) in abc.iter() {
        edge.push((Reverse(c), a, b));
    }
    edge.sort_unstable();

    let mut uf = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut ans = 0;
    for &(Reverse(c), a, b) in edge.iter() {
        if !is_same(&mut uf, a, b) {
            ans += c;
            merge(&mut uf, &mut rank, a, b);
        }
    }
    println!("{}", ans);
}
