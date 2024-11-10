use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        s: Chars
    );

    let mut ans = 0;
    let mut cur = 0;
    for (i, &si) in s.iter().enumerate() {
        if si as u8 == b'A' {
            cur = i;
        }
    }

    for i in 1..26 {
        for (j, &sj) in s.iter().enumerate() {
            if sj as u8 == b'A' + i {
                ans += cur.abs_diff(j);
                cur = j;
            }
        }
    }
    println!("{}", ans);
}
