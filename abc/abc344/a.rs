use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
    );
    let mut p = true;
    for &si in &s {
        if si == '|' {
            p = !p;
            continue;
        }
        if p {
            print!("{}", si);
        }
    }
}
