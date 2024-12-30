use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };
    if !s[0].is_uppercase() {
        println!("No");
        return;
    }
    for &si in s.iter().skip(1) {
        if !si.is_lowercase() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
