use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k, x): (usize, usize, usize),
        mut a: [usize; n],
    );
    a.insert(k, x);
    println!("{}", a.iter().join(" "));
}
