use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
        t: Chars,
    );
    if s.len() == 1 {
        println!("No");
        return;
    }
    for w in 1..s.len() {
        for c in 1..=w {
            let mut i = 0;
            let mut v = Vec::new();
            while i * w + c - 1 < s.len() {
                v.push(s[i * w + c - 1]);
                i += 1;
            }
            if v == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
