use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (n, d): (usize, usize),
        s: Chars,
    );
    let mut c = 0;
    for &si in &s {
        if si == '@' {
            c += 1
        }
    }

    let ans = (n - c) + d;
    println!("{}", ans);
}
