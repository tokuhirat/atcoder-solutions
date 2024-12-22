use proconio::{fastout, input, marker::Usize1};
use std::{cmp::Reverse, collections::BTreeMap};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut tree = vec![vec![]; n];
    for _ in 0..n - 1 {
        input!(
            (u, v): (Usize1, Usize1),
        );
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut ans = usize::MAX;
    for root in 0..n {
        let nx = &tree[root];
        let mut cand = BTreeMap::new();
        for &x in nx {
            let c = tree[x].len();
            if c == 1 {
                continue;
            }
            let e = cand.entry(Reverse(c)).or_insert(0);
            *e += 1;
        }
        let mut tree_size = 0;
        let mut cnt = 0;
        for (&Reverse(c), &num) in cand.iter() {
            cnt += num;
            tree_size = tree_size.max(c * cnt + 1);
        }
        ans = ans.min(n - tree_size);
    }
    println!("{}", ans);
}
