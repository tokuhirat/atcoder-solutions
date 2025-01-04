use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    };
    println!("{}", (a + b) * (a + b));
}
