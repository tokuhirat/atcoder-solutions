use proconio::{fastout, input, marker::Usize1};
use std::collections::BinaryHeap;

const M_INF: i64 = i64::MIN;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    };
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {
            (l, d, k, c, a, b): (i64, i64, i64, i64, Usize1, Usize1)
        };
        graph[b].push((l, d, k, c, a));
    }

    let mut bh = BinaryHeap::new();
    for &(l, d, k, _, a) in &graph[n - 1] {
        let t = l + (k - 1) * d;
        bh.push((t, a));
    }

    let mut time = vec![M_INF; n];
    while let Some((t, b)) = bh.pop() {
        if time[b] == M_INF {
            time[b] = t;
        }
        for &(l, d, k, c, a) in &graph[b] {
            if l + c > t {
                continue;
            }
            let s = t - c - l;
            let mut num = s / d;
            if num >= k {
                num = k - 1;
            }
            if l + num * d >= time[a] {
                bh.push((l + num * d, a));
            }
        }
    }
    for &ti in time.iter().take(n - 1) {
        if ti != M_INF {
            println!("{}", ti);
        } else {
            println!("Unreachable");
        }
    }
}
