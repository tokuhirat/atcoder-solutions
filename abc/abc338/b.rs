use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    };
    let mut v = [0; 26];
    for &si in &s {
        let i = (si as u8 - b'a') as usize;
        v[i] += 1;
    }
    let mut max = 0;
    let mut ans = 0;
    for (i, &cnt) in v.iter().enumerate() {
        if cnt > max {
            ans = i;
            max = cnt;
        }
    }
    println!("{}", (ans as u8 + b'a') as char);
}
