use proconio::{fastout, input, marker::Usize1};

fn dfs1(tree: &[Vec<usize>], c: &[usize], v: usize, p: usize, x: &mut usize, s: usize) -> usize {
    let mut ret = c[v];
    let mut max_size = 0;
    for &nxt in &tree[v] {
        if nxt == p {
            continue;
        }
        let child = dfs1(tree, c, nxt, v, x, s);
        max_size = max_size.max(child);
        ret += child;
    }
    max_size = max_size.max(s - ret);
    if max_size * 2 <= s {
        *x = v;
    }
    ret
}

fn dfs2(tree: &[Vec<usize>], c: &[usize], v: usize, p: usize, d: usize, ans: &mut usize) {
    *ans += d * c[v];
    for &nxt in &tree[v] {
        if nxt == p {
            continue;
        }
        dfs2(tree, c, nxt, v, d + 1, ans);
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut tree = vec![vec![]; n];
    for _ in 0..n - 1 {
        input!((a, b): (Usize1, Usize1));
        tree[a].push(b);
        tree[b].push(a);
    }

    input!(
        c: [usize; n],
    );
    let s: usize = c.iter().sum();
    let mut x = 0;
    dfs1(&tree, &c, 0, usize::MAX, &mut x, s);
    // println!("{}", x);
    let mut ans = 0;
    dfs2(&tree, &c, x, usize::MAX, 0, &mut ans);
    println!("{}", ans);
}
