use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        (h, w, y): (usize, usize, usize),
        a: [[usize; w]; h],
    );

    let mut q = vec![VecDeque::new(); 100001];
    let mut over = vec![vec![true; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                q[a[i][j]].push_back((i, j));
                over[i][j] = false;
            }
        }
    }

    let mut ans = h * w;
    let d: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 1..=y {
        while let Some((i, j)) = q[y].pop_front() {
            ans -= 1;
            let ii = i as isize;
            let jj = j as isize;
            for (di, dj) in d {
                let nxt_i = ii + di;
                let nxt_j = jj + dj;
                if nxt_i >= 0 && nxt_i < h as isize && nxt_j >= 0 && nxt_j < w as isize {
                    let nxt_i = nxt_i as usize;
                    let nxt_j = nxt_j as usize;
                    if over[nxt_i][nxt_j] {
                        q[a[nxt_i][nxt_j].max(y)].push_back((nxt_i, nxt_j));
                        over[nxt_i][nxt_j] = false;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
