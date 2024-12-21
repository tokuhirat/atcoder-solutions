use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
    );
    if s.first().unwrap() != &'<' || s.last().unwrap() != &'>' {
        println!("No");
        return;
    }
    for &si in s.iter().take(s.len() - 1).skip(1) {
        if si != '=' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
