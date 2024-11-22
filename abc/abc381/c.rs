use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars
    );
    let mut idx = vec![];
    for (i, &si) in s.iter().enumerate() {
        if si == '/' {
            idx.push(i);
        }
    }
    let mut ans = 1;
    for &i in &idx {
        let mut l = 1;
        while i >= l && i + l < n && s[i - l] == '1' && s[i + l] == '2' {
            l += 1;
        }
        ans = ans.max(l);
    }
    println!("{}", 2 * ans - 1);
}
