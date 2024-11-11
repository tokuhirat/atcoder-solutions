use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        n: usize,
        m_g: usize,
        uv: [(Usize1, Usize1); m_g],
        m_h: usize,
        ab: [(Usize1, Usize1); m_h],
    );
    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input!(
            tmp_a: [usize; n - i - 1],
        );
        for (j, &ai) in tmp_a.iter().enumerate() {
            a[i][i + j + 1] = ai;
            a[i + j + 1][i] = ai;
        }
    }

    let mut g = vec![HashSet::new(); n];
    for &(u, v) in uv.iter() {
        g[u].insert(v);
        g[v].insert(u);
    }
    let mut h = vec![HashSet::new(); n];
    for &(a, b) in ab.iter() {
        h[a].insert(b);
        h[b].insert(a);
    }

    let mut ans = usize::MAX;
    for p in (0..n).permutations(n) {
        let mut cost = 0;
        for from_to in (0..n).combinations(2) {
            let from_h = *from_to.first().unwrap();
            let to_h = *from_to.last().unwrap();
            let from_g = p[from_h];
            let to_g = p[to_h];

            if (g[from_g].contains(&to_g) && !h[from_h].contains(&to_h))
                || (!g[from_g].contains(&to_g) && h[from_h].contains(&to_h))
            {
                cost += a[from_h][to_h];
            }
        }
        if cost < ans {
            ans = cost;
        }
    }
    println!("{}", ans);
}
