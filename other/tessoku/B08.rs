use proconio::{fastout, input};

#[fastout]
#[allow(clippy::needless_range_loop)]
fn main() {
    input!(
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    );

    let max_size = 1502;
    let mut count = vec![vec![0; max_size]; max_size];
    for &(x, y) in xy.iter() {
        count[x][y] += 1;
    }

    for i in 1..max_size {
        for j in 1..max_size {
            count[i][j] += count[i][j - 1];
        }
    }
    for j in 1..max_size {
        for i in 1..max_size {
            count[i][j] += count[i - 1][j];
        }
    }

    for &(a, b, c, d) in abcd.iter() {
        let ans = count[c][d] - count[c][b - 1] - count[a - 1][d] + count[a - 1][b - 1];
        println!("{}", ans);
    }
}
