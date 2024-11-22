use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars
    );
    if n % 2 == 0 {
        println!("No");
        return;
    }
    for i in 0..(n + 1) / 2 - 1 {
        if s[i] != '1' || s[n - i - 1] != '2' {
            println!("No");
            return;
        }
    }
    if s[(n + 1) / 2 - 1] == '/' {
        println!("Yes");
    } else {
        println!("No");
    }
}
