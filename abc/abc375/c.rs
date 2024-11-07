use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [Chars; n],
    );
    let mut b = a.clone();
    for i in 0..n / 2 {
        match (i + 1) % 4 {
            0 => {}
            1 => {
                for j in i..n - i {
                    b[j][n - i - 1] = a[i][j];
                    b[n - i - 1][n - j - 1] = a[j][n - i - 1];
                    b[n - j - 1][i] = a[n - i - 1][n - j - 1];
                    b[i][j] = a[n - j - 1][i];
                }
            }
            2 => {
                for j in i..n - i {
                    b[n - i - 1][n - j - 1] = a[i][j];
                    b[n - j - 1][i] = a[j][n - i - 1];
                    b[i][j] = a[n - i - 1][n - j - 1];
                    b[j][n - i - 1] = a[n - j - 1][i];
                }
            }
            3 => {
                for j in i..n - i {
                    b[n - j - 1][i] = a[i][j];
                    b[i][j] = a[j][n - i - 1];
                    b[j][n - i - 1] = a[n - i - 1][n - j - 1];
                    b[n - i - 1][n - j - 1] = a[n - j - 1][i];
                }
            }
            _ => unreachable!(),
        };
    }
    for bi in b.iter() {
        for &bij in bi.iter() {
            print!("{}", bij);
        }
        println!("");
    }
}
