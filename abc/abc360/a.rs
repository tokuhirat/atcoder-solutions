use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    if s[0] == 'R' {
        println!("Yes");
        return;
    }
    if s[1] == 'R' && s[2] == 'M' {
        println!("Yes");
        return;
    }
    println!("No");
}
