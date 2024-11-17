use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, t, a): (usize, usize, usize),
    );
    if t > (n - 1) / 2 || a > (n - 1) / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
