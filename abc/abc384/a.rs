use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        _n: usize,
        (c1, c2): (char, char),
        s: Chars,
    );
    for &si in &s {
        if si == c1 {
            print!("{}", si);
        } else {
            print!("{}", c2);
        }
    }
    println!("");
}
