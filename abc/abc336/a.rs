use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    print!("L");
    for _ in 0..n {
        print!("o");
    }
    print!("n");
    println!("g");
}
