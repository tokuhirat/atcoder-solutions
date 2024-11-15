use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (a, b, c): (usize, usize, usize),
    );
    if c < b {
        if c < a && a < b {
            println!("Yes");
        } else {
            println!("No")
        }
    } else if b < a && a < c {
        println!("No")
    } else {
        println!("Yes");
    }
}
