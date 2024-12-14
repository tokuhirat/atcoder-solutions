use proconio::{fastout, input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap, collections::HashSet};

#[fastout]
fn main() {
    input!(
        (h, w, x): (usize, usize, usize),
        (p, q): (Usize1, Usize1),
        s: [[usize; w]; h],
    );
    let mut queue = BinaryHeap::new();
    let d = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for di in &d {
        let (ni, nj) = (p as i32 + di.0, q as i32 + di.1);
        if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
            continue;
        }
        let (ni, nj) = (ni as usize, nj as usize);
        queue.push((Reverse(s[ni][nj]), ni, nj));
    }
    let mut point = s[p][q];
    let mut tk = HashSet::new();
    tk.insert((p, q));
    while let Some((Reverse(sij), i, j)) = queue.pop() {
        if sij.saturating_mul(x) >= point {
            break;
        }
        if tk.contains(&(i, j)) {
            continue;
        }
        point += sij;
        tk.insert((i, j));

        for di in &d {
            let (ni, nj) = (i as i32 + di.0, j as i32 + di.1);
            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if tk.contains(&(ni, nj)) {
                continue;
            }
            queue.push((Reverse(s[ni][nj]), ni, nj));
        }
    }
    println!("{}", point);
}
