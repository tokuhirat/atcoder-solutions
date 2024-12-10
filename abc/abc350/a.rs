use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
    );
    let s = 100 * s[3].to_digit(10).unwrap()
        + 10 * s[4].to_digit(10).unwrap()
        + s[5].to_digit(10).unwrap();
    if s > 0 && s <= 349 && s != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
