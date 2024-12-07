use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        (h, w, d): (usize, usize, usize),
        s: [Chars; h],
    );
    let mut queue = VecDeque::new();
    let mut grid = vec![vec![usize::MAX; w]; h];
    for (i, si) in s.iter().enumerate() {
        for (j, sij) in si.iter().enumerate() {
            if sij == &'H' {
                queue.push_back((i, j));
                grid[i][j] = 0;
            }
        }
    }
    let direction = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in &direction {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni >= 0 && ni < h as i32 && nj >= 0 && nj < w as i32 {
                let (ni, nj) = (ni as usize, nj as usize);
                if s[ni][nj] == '#' {
                    continue;
                }
                if grid[ni][nj] > grid[i][j] + 1 {
                    queue.push_back((ni, nj));
                    grid[ni][nj] = grid[i][j] + 1;
                }
            }
        }
    }
    let mut cnt = 0;
    for gi in &grid {
        for &gij in gi {
            if gij <= d {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
