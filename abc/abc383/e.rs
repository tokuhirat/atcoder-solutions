use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, m, k): (usize, usize, usize),
    );
    let mut graph = Vec::with_capacity(m);
    for _ in 0..m {
        input!((u, v, w): (Usize1, Usize1, usize));
        graph.push((w, u, v));
    }
    graph.sort_unstable();

    let mut a = vec![0; n];
    for _ in 0..k {
        input!(i: Usize1);
        a[i] += 1;
    }
    let mut b = vec![0; n];
    for _ in 0..k {
        input!(i: Usize1);
        b[i] += 1;
    }

    let mut uf = Dsu::new(n);
    let mut ans = 0;
    for &(w, u, v) in &graph {
        if uf.same(u, v) {
            continue;
        }
        let ru = uf.leader(u);
        let rv = uf.leader(v);
        uf.merge(u, v);
        let new_root = uf.leader(ru);
        a[new_root] = a[ru] + a[rv];
        b[new_root] = b[ru] + b[rv];

        let c = a[new_root].min(b[new_root]);
        ans += c * w;
        a[new_root] -= c;
        b[new_root] -= c;
    }
    println!("{}", ans);
}
