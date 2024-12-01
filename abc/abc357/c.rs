use proconio::{fastout, input};

fn dfs(n: usize) -> Vec<Vec<char>> {
    if n == 0 {
        return vec![vec!['#']];
    }
    let a = dfs(n - 1);
    let k = 3_usize.pow(n as u32);
    let mut ret = vec![vec!['.'; k]; k];
    let k = k / 3;
    for i in 0..3_usize.pow(n as u32 - 1) {
        for j in 0..3_usize.pow(n as u32 - 1) {
            ret[i][j] = a[i][j];
            ret[i + k][j] = a[i][j];
            ret[i + 2 * k][j] = a[i][j];
            ret[i][j + k] = a[i][j];
            ret[i][j + 2 * k] = a[i][j];
            ret[i + k][j + 2 * k] = a[i][j];
            ret[i + 2 * k][j + k] = a[i][j];
            ret[i + 2 * k][j + 2 * k] = a[i][j];
        }
    }
    ret
}

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let k = dfs(n);
    for ki in k {
        for kij in ki {
            print!("{}", kij);
        }
        println!("");
    }
}
