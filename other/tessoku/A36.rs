use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
    );

    if k < 2 * n - 2 || (k - (2 * n - 2)) % 2 == 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
