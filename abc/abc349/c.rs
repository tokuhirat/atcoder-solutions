use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
        t: Chars
    );
    let mut i = 0;
    for &si in &s {
        if si == t[i].to_ascii_lowercase() {
            if i == 2 {
                println!("Yes");
                return;
            }
            i += 1;
        }
    }
    if i == 2 && t[2] == 'X' {
        println!("Yes");
    } else {
        println!("No");
    }
}
