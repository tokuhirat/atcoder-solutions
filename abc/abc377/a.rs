use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        mut s:Chars,
    );

    s.sort();
    if s == ['A', 'B', 'C'] {
        println!("Yes");
    } else {
        println!("No");
    }
}