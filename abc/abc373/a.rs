use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: [Chars; 12],
    );

    let mut ans = 0;
    for (i, si) in s.iter().enumerate() {
        if i + 1 == si.len() {
            ans += 1;
        }
    }
    println!("{}", ans);
}
