use proconio::{fastout, input};
const MAX: usize = 1500;

#[fastout]
#[allow(clippy::needless_range_loop)]
fn main() {
    input!(
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    );

    let mut count = vec![vec![0; MAX + 1]; MAX + 1];
    for &(a, b, c, d) in abcd.iter() {
        count[a][b] += 1;
        count[a][d] -= 1;
        count[c][b] -= 1;
        count[c][d] += 1;
    }

    for x in 0..=MAX {
        for y in 1..=MAX {
            count[x][y] += count[x][y - 1];
        }
    }
    for y in 0..=MAX {
        for x in 1..=MAX {
            count[x][y] += count[x - 1][y];
        }
    }

    let mut ans = 0;
    for x in 0..=MAX {
        for y in 0..=MAX {
            if count[x][y] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
