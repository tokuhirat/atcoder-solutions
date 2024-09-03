use proconio::input;
fn main() {
    input!(
        h: usize, w: usize,
        x: [[u32; w]; h],
        q: u32,
        abcd: [(usize, usize, usize, usize); q]
    );

    let mut x_acc = vec![vec![0; w + 1]; h + 1];
    for hi in 1..=h {
        for wj in 1..=w {
            x_acc[hi][wj] = x_acc[hi][wj - 1] + x[hi - 1][wj - 1];
        }
    }

    for hi in 1..=h {
        for wj in 1..=w {
            x_acc[hi][wj] += x_acc[hi - 1][wj];
        }
    }

    for (a, b, c, d) in abcd {
        let sum = x_acc[c][d] - x_acc[a - 1][d] - x_acc[c][b - 1] + x_acc[a - 1][b - 1];
        println!("{sum}");
    }
}
