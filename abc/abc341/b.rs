use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    for i in 0..n - 1 {
        input! {(s, t): (usize, usize)};
        a[i + 1] += t * (a[i] / s);
    }
    println!("{}", a.last().unwrap());
}
