use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

fn bfs(s: &[Vec<char>], start: (usize, usize), d: usize) -> usize {
    let h = s.len();
    let w = s[0].len();
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![usize::MAX; w]; h];
    queue.push_back((start, d, 0));
    dist[start.0][start.1] = 0;
    let dir = [[(0, 1), (0, -1)], [(1, 0), (-1, 0)]];

    while let Some(((i, j), d, c)) = queue.pop_front() {
        for (di, dj) in dir[d] {
            let ni = i.wrapping_add_signed(di);
            let nj = j.wrapping_add_signed(dj);
            if ni >= h || nj >= w {
                continue;
            }
            if s[ni][nj] == '#' || dist[ni][nj] != usize::MAX {
                continue;
            }
            if s[ni][nj] == 'G' {
                return c + 1;
            }
            queue.push_back(((ni, nj), d ^ 1, c + 1));
            dist[ni][nj] = 0;
        }
    }
    usize::MAX
}

#[fastout]
fn main() {
    input! {
        (h, _w): (usize, usize),
        s: [Chars; h],
    };
    let mut start = (usize::MAX, usize::MAX);
    for (i, si) in s.iter().enumerate() {
        for (j, &sij) in si.iter().enumerate() {
            if sij == 'S' {
                start = (i, j);
            }
        }
    }
    let ans = bfs(&s, start, 0).min(bfs(&s, start, 1));

    if ans != usize::MAX {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
