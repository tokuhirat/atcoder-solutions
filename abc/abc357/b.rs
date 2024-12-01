use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    let mut upper = 0;
    for &si in &s {
        if si.is_uppercase() {
            upper += 1;
        }
    }
    if 2 * upper > s.len() {
        for &si in &s {
            print!("{}", si.to_ascii_uppercase())
        }
        println!("");
    } else {
        for &si in &s {
            print!("{}", si.to_ascii_lowercase())
        }
        println!("");
    }
}
