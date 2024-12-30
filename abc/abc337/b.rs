use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut i = 0;
    while i < s.len() && s[i] == 'A' {
        i += 1;
    }
    while i < s.len() && s[i] == 'B' {
        i += 1;
    }
    while i < s.len() && s[i] == 'C' {
        i += 1;
    }
    if i == s.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
