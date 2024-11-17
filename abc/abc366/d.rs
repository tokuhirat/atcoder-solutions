use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [[[usize; n]; n]; n],
        q: usize,
    );
    let mut a_cum = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 0..n {
        for j in 1..=n {
            for k in 1..=n {
                a_cum[i + 1][j][k] = a_cum[i][j][k] + a[i][j - 1][k - 1];
            }
        }
    }
    for i in 1..=n {
        for j in 0..n {
            for k in 1..=n {
                a_cum[i][j + 1][k] += a_cum[i][j][k];
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            for k in 0..n {
                a_cum[i][j][k + 1] += a_cum[i][j][k];
            }
        }
    }

    for _ in 0..q {
        input!(
            (lx, rx, ly, ry, lz, rz): (usize, usize, usize, usize, usize, usize),
        );
        let ans = a_cum[rx][ry][rz] - a_cum[lx - 1][ly - 1][lz - 1]
            + a_cum[rx][ly - 1][lz - 1]
            + a_cum[lx - 1][ry][lz - 1]
            + a_cum[lx - 1][ly - 1][rz]
            - a_cum[lx - 1][ry][rz]
            - a_cum[rx][ly - 1][rz]
            - a_cum[rx][ry][lz - 1];

        println!("{}", ans);
    }
}
