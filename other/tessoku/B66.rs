use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

fn union(uf: &mut [usize], rank: &mut [usize], size: &mut [usize], a: usize, b: usize) {
    let mut root_a = root(uf, a);
    let mut root_b = root(uf, b);
    if rank[root_a] > rank[root_b] {
        (root_a, root_b) = (root_b, root_a);
    }
    uf[root_a] = root_b;
    if rank[root_a] == rank[root_b] {
        rank[root_b] += 1;
    }
    size[root_b] += size[root_a];
}

fn root(uf: &mut [usize], x: usize) -> usize {
    if uf[x] == x {
        return x;
    }
    uf[x] = root(uf, uf[x]);
    uf[x]
}

fn is_same(uf: &mut [usize], a: usize, b: usize) -> bool {
    root(uf, a) == root(uf, b)
}

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
        q: usize,
    );

    let mut to_be_removed = HashSet::new();
    let mut query = vec![];
    for _ in 0..q {
        input!(t: usize);
        if t == 1 {
            input!(x: Usize1);
            to_be_removed.insert(x);
            query.push((t, x, x));
        } else {
            input!(
                (u, v): (Usize1, Usize1)
            );
            query.push((t, u, v));
        }
    }

    let mut uf = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut size = vec![1; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        if to_be_removed.contains(&i) {
            continue;
        }
        union(&mut uf, &mut rank, &mut size, a, b);
    }

    let mut ans = vec![];
    for &(t, x, y) in query.iter().rev() {
        if t == 1 {
            union(&mut uf, &mut rank, &mut size, ab[x].0, ab[x].1);
        } else {
            ans.push(is_same(&mut uf, x, y));
        }
    }

    for &a in ans.iter().rev() {
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
