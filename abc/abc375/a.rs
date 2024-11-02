use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars,
    );

    if n <= 2 {
        println!("0");
        return;
    }

    let mut ans = 0;
    for i in 0..n - 2 {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
