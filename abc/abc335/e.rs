use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize; n],
    };
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        input! {(u, v): (Usize1, Usize1)};
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut queue = BinaryHeap::new();
    let mut d = vec![0; n];
    queue.push((Reverse(a[0]), 0, 1));
    d[0] = 1;
    while let Some((_, from, cnt)) = queue.pop() {
        if d[from] != cnt {
            continue;
        }
        for &nxt in &graph[from] {
            match a[nxt].cmp(&a[from]) {
                std::cmp::Ordering::Greater => {
                    if d[nxt] < cnt + 1 {
                        queue.push((Reverse(a[nxt]), nxt, cnt + 1));
                        d[nxt] = cnt + 1;
                    }
                }
                std::cmp::Ordering::Equal => {
                    if d[nxt] < cnt {
                        queue.push((Reverse(a[nxt]), nxt, cnt));
                        d[nxt] = cnt;
                    }
                }
                std::cmp::Ordering::Less => (),
            }
        }
    }
    println!("{}", d[n - 1]);
}
