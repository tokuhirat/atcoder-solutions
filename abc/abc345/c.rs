use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars,
    );
    let mut c = [0; 26];
    let mut ans = 0;
    for j in 0..s.len() {
        if j != 0 {
            ans += j - c[(s[j] as u8 - b'a') as usize];
        }
        c[(s[j] as u8 - b'a') as usize] += 1;
    }
    for &ci in &c {
        if ci > 1 {
            ans += 1;
            break;
        }
    }
    println!("{}", ans);
}
