use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    };

    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        if i < s.len() - 1 && s[i] == '0' && s[i + 1] == '0' {
            ans += 1;
            i += 2;
        } else {
            ans += 1;
            i += 1;
        }
    }
    println!("{}", ans);
}
