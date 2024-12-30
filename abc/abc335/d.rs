use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut grid = vec![vec![String::from("T"); n]; n];
    let mut l = n - 1;
    let mut cnt = 0;
    let mut p = 1;
    while l > 0 {
        for i in 0..l {
            grid[cnt][cnt + i] = p.to_string();
            p += 1;
        }
        for i in 0..l {
            grid[cnt + i][n - cnt - 1] = p.to_string();
            p += 1;
        }
        for i in 0..l {
            grid[n - cnt - 1][cnt + l - i] = p.to_string();
            p += 1;
        }
        for i in 0..l {
            grid[cnt + l - i][cnt] = p.to_string();
            p += 1;
        }
        l -= 2;
        cnt += 1;
    }

    for g in &grid {
        println!("{}", g.iter().join(" "));
    }
}
