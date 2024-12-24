use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (a, b): (usize, usize),
    );
    for i in 0..=9 {
        if i != a + b {
            println!("{}", i);
            return;
        }
    }
}
