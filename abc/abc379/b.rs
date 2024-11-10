use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (_n, k): (usize, usize),
        s: Chars,
    );

    let mut ans = 0;
    let mut cur = 0;
    for &si in s.iter() {
        if si == 'O' {
            cur += 1;
            if cur == k {
                ans += 1;
                cur = 0;
            }
        } else {
            cur = 0;
        }
    }
    println!("{}", ans);
}
