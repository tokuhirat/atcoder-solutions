use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        r: usize,
        c: usize,
        (sy, sx): (i32, i32),
        (gy, gx): (i32, i32),
        table: [Chars; r],
    );

    let (sy, sx) = (sy - 1, sx - 1);
    let (gy, gx) = (gy - 1, gx - 1);

    let direction = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::from([(sy, sx)]);
    let mut dist = vec![vec![-1; c]; r];
    dist[sy as usize][sx as usize] = 0;

    while let Some(cur) = queue.pop_front() {
        if cur.0 == gy && cur.1 == gx {
            break;
        }
        for &d in direction.iter() {
            let nxt_y = cur.0 + d.0;
            let nxt_x = cur.1 + d.1;

            if 0 <= nxt_y
                && nxt_y < r as i32
                && 0 <= nxt_x
                && nxt_x < c as i32
                && dist[nxt_y as usize][nxt_x as usize] == -1
                && table[nxt_y as usize][nxt_x as usize] == '.'
            {
                dist[nxt_y as usize][nxt_x as usize] = dist[cur.0 as usize][cur.1 as usize] + 1;
                queue.push_back((nxt_y, nxt_x));
            }
        }
    }
    println!("{}", dist[gy as usize][gx as usize]);
}
