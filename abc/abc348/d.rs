use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        (h, w): (usize, usize),
        a: [Chars; h],
        n: usize,
    );

    let (mut si, mut sj) = (0, 0);
    let (mut ti, mut tj) = (0, 0);
    for (i, ai) in a.iter().enumerate() {
        for (j, aij) in ai.iter().enumerate() {
            if aij == &'S' {
                (si, sj) = (i, j);
            }
            if aij == &'T' {
                (ti, tj) = (i, j);
            }
        }
    }

    let mut meds = vec![];
    meds.push((si, sj, 0));
    for _ in 0..n {
        input!((r, c, e): (Usize1, Usize1, usize));
        meds.push((r, c, e));
    }
    meds.push((ti, tj, 0));
    let n = n + 2;

    let d = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut graph = vec![vec![]; n];
    for (u, &(i, j, e)) in meds.iter().enumerate() {
        let mut dist = vec![vec![usize::MAX; w]; h];
        let mut queue = VecDeque::new();
        queue.push_back((i, j));
        dist[i][j] = 0;
        while let Some((i, j)) = queue.pop_front() {
            for di in &d {
                let ni = i as i32 + di.0;
                let nj = j as i32 + di.1;
                if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if a[ni][nj] == '#' {
                    continue;
                }
                if dist[ni][nj] != usize::MAX {
                    continue;
                }

                dist[ni][nj] = dist[i][j] + 1;
                queue.push_back((ni, nj));
            }
        }
        for (v, &(ni, nj, _)) in meds.iter().enumerate() {
            if u == v {
                continue;
            }
            if dist[ni][nj] <= e {
                graph[u].push(v);
            }
        }
    }

    let mut dist = vec![usize::MAX; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    dist[0] = 0;
    while let Some(i) = queue.pop_front() {
        for &nxt in &graph[i] {
            if dist[nxt] != usize::MAX {
                continue;
            }
            dist[nxt] = dist[i] + 1;
            queue.push_back(nxt);
        }
    }
    if dist[n - 1] != usize::MAX {
        println!("Yes");
    } else {
        println!("No");
    }
}
