use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
    };
    let mut grid = vec![vec!['.'; w]; h];
    let (mut i, mut j, mut d) = (0, 0, 0);
    for _ in 0..n {
        if grid[i][j] == '.' {
            grid[i][j] = '#';
            d = (d + 1) % 4;
        } else {
            grid[i][j] = '.';
            d = (d + 3) % 4;
        }
        match d {
            0 => {
                i += h - 1;
                i %= h;
            }
            1 => {
                j += 1;
                j %= w;
            }
            2 => {
                i += 1;
                i %= h;
            }
            3 => {
                j += w - 1;
                j %= w;
            }
            _ => unreachable!(),
        }
    }
    for gi in &grid {
        println!("{}", gi.iter().join(""));
    }
}
