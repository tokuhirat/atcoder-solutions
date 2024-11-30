use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        mut a: [usize; n],
        b: [usize; m],
    );

    for i in 1..n {
        a[i] = a[i - 1].min(a[i])
    }

    for bi in &b {
        let i = a.partition_point(|e| e > bi);
        if i == n {
            println!("-1");
        } else {
            println!("{}", i + 1);
        }
    }
}
