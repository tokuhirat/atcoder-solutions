use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        _n: usize,
        k: usize,
        s: Chars
    );

    let o = s
        .iter()
        .fold(0, |acc, &e| acc + if e == '1' { 1 } else { 0 });

    if o % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
