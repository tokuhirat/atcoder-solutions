use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        input!(
            ai: [Usize1; i + 1],
        );
        for (j, &aij) in ai.iter().enumerate() {
            a[i][j] = aij;
            a[j][i] = aij;
        }
    }

    let mut e = 0;
    for i in 0..n {
        e = a[e][i];
    }
    println!("{}", e + 1);
}
