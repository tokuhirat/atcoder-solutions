use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        s: Chars
    );
    let mut l_0 = 0;
    let mut l_1 = 0;
    let mut r_1 = 0;

    let mut i = 0;
    let mut c = 0;
    while i < n {
        if s[i] == '1' {
            c += 1;
            l_1 = i;
            while i + 1 < n && s[i + 1] == '1' {
                i += 1;
            }
            r_1 = i;
            if c == k - 1 {
                l_0 = r_1 + 1;
            }
            if c == k {
                break;
            }
        }
        i += 1;
    }
    let mut ans = vec![];
    for (i, &si) in s.iter().enumerate() {
        if i < l_0 || r_1 < i {
            ans.push(si);
        } else if i - l_0 <= r_1 - l_1 {
            ans.push('1');
        } else {
            ans.push('0');
        }
    }
    for ai in &ans {
        print!("{}", ai);
    }
    println!("");
}
