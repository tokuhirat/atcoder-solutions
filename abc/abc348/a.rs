use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    for i in 0..n {
        if (i + 1) % 3 != 0 {
            print!("o")
        } else {
            print!("x")
        }
    }
    println!("");
}
