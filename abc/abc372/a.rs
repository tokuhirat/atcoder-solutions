use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );
    for &si in s.iter() {
        if si != '.' {
            print!("{}", si);
        }
    }
    println!("")
}
