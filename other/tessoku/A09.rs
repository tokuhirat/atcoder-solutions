use proconio::input;
fn main() {
    input!(
        h: usize, w: usize, n: usize,
        abcd: [(usize, usize, usize, usize); n]
    );

    let mut area = vec![vec![0; w + 1]; h + 1];

    for &(a, b, c, d) in abcd.iter() {
        area[a - 1][b - 1] += 1;
        area[a - 1][d] -= 1;
        area[c][b - 1] -= 1;
        area[c][d] += 1;
    }

    for hi in 0..h {
        for wj in 0..w {
            area[hi][wj + 1] += area[hi][wj];
        }
    }

    for hi in 0..h {
        for wj in 0..w {
            area[hi + 1][wj] += area[hi][wj];
        }
    }

    for hi in 0..h {
        for wj in 0..w {
            print!("{}", area[hi][wj]);
            if wj == w - 1 {
                println!("");
            } else {
                print!(" ");
            }
        }
    }
}
