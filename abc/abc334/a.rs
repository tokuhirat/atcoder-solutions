use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (b, g): (usize, usize),
    };
    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}
