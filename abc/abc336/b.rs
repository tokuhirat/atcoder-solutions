use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    println!("{}", n.trailing_zeros());
}
