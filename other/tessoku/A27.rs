use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut a: usize,
        mut b: usize
    );

    while a % b != 0 {
        (a, b) = (b, a % b);
    }

    println!("{}", b);
}
