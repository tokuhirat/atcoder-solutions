use proconio::{fastout, input, marker::Chars};

fn dfs(
    table: &[Vec<char>],
    h: usize,
    w: usize,
    visited: &mut [Vec<bool>],
    row: usize,
    col: usize,
    step: usize,
) -> usize {
    if table[row][col] == '#' {
        return 0;
    }
    if step == 0 {
        return 1;
    }

    visited[row][col] = true;
    let mut num = 0;
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nxt_row = row as i32 + d.0;
        let nxt_col = col as i32 + d.1;
        if 0 <= nxt_row && nxt_row < h as i32 && 0 <= nxt_col && nxt_col < w as i32 {
            let nxt_row = nxt_row as usize;
            let nxt_col = nxt_col as usize;
            if !visited[nxt_row][nxt_col] {
                num += dfs(table, h, w, visited, nxt_row, nxt_col, step - 1);
            }
        }
    }
    visited[row][col] = false;
    num
}

#[fastout]
fn main() {
    input!(
        (h, w, k): (usize, usize, usize),
        s: [Chars; h],
    );

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut visited = vec![vec![false; w]; h];
            ans += dfs(&s, h, w, &mut visited, i, j, k);
        }
    }
    println!("{}", ans);
}
