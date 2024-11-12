use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (l, r): (u8, u8),
    );
    if l == 1 && r == 0 {
        println!("Yes");
        return;
    }
    if r == 1 && l == 0 {
        println!("No");
        return;
    }
    println!("Invalid");
}
