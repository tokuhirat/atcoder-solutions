use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (mut a, b, d): (usize, usize, usize),
    };
    while a <= b {
        print!("{} ", a);
        a += d;
    }
    println!("");
}
