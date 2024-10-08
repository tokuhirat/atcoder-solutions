use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars,
    );

    let mut ans = false;
    for i in 0..n - 2 {
        if s[i] == s[i + 1] && s[i + 1] == s[i + 2] && s[i + 2] == 'B' {
            ans = true;
            break;
        }
        if s[i] == s[i + 1] && s[i + 1] == s[i + 2] && s[i + 2] == 'R' {
            ans = true;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
