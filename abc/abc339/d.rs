use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

fn nxt(p: i32, n: usize) -> usize {
    let mut r = 0;
    if p > 0 {
        r = p as usize;
    }
    if r >= n {
        r -= 1;
    }
    r
}

fn nxt_p(p: (i32, i32), n: usize) -> (usize, usize) {
    (nxt(p.0, n), nxt(p.1, n))
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };
    let mut p1 = (usize::MAX, usize::MAX);
    let mut p2 = (usize::MAX, usize::MAX);
    for (i, si) in s.iter_mut().enumerate() {
        for (j, sij) in si.iter_mut().enumerate() {
            if *sij == 'P' {
                if p1.0 == usize::MAX {
                    p1 = (i, j);
                    *sij = '.';
                } else {
                    p2 = (i, j);
                    *sij = '.';
                    break;
                }
            }
        }
    }
    let mut dist = vec![vec![vec![vec![usize::MAX; n]; n]; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back((p1, p2));
    dist[p1.0][p1.1][p2.0][p2.1] = 0;
    while let Some((a, b)) = queue.pop_front() {
        if a == b {
            println!("{}", dist[a.0][a.1][b.0][b.1]);
            return;
        }

        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let n_a = (a.0 as i32 + dir.0, a.1 as i32 + dir.1);
            let n_b = (b.0 as i32 + dir.0, b.1 as i32 + dir.1);
            let mut n_a = nxt_p(n_a, n);
            let mut n_b = nxt_p(n_b, n);
            if s[n_a.0][n_a.1] == '#' {
                n_a = a;
            }
            if s[n_b.0][n_b.1] == '#' {
                n_b = b;
            }
            if dist[n_a.0][n_a.1][n_b.0][n_b.1] != usize::MAX {
                continue;
            }
            dist[n_a.0][n_a.1][n_b.0][n_b.1] = dist[a.0][a.1][b.0][b.1] + 1;
            queue.push_back((n_a, n_b));
        }
    }
    println!("-1");
}
