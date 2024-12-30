use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };
    for &si in s.iter().take(s.len() - 1) {
        print!("{}", si);
    }
    println!("4");
}
