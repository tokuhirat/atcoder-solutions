use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        n: usize,
        s: Chars,
        c: [usize; n],
    );
    let mut c_01 = Vec::with_capacity(n + 1);
    let mut c_10 = Vec::with_capacity(n + 1);
    c_01.push(0);
    c_10.push(0);
    for (i, (&si, &ci)) in s.iter().zip(c.iter()).enumerate() {
        if (i % 2 == 0 && si == '1') || (i % 2 != 0 && si == '0') {
            c_01.push(c_01.last().unwrap() + ci);
        } else {
            c_01.push(*c_01.last().unwrap());
        }

        if (i % 2 == 0 && si == '0') || (i % 2 != 0 && si == '1') {
            c_10.push(c_10.last().unwrap() + ci);
        } else {
            c_10.push(*c_10.last().unwrap());
        }
    }

    let mut ans = usize::MAX;
    for i in 0..n - 1 {
        ans = ans.min(c_01[i + 1] + c_10[n] - c_10[i + 1]);
        ans = ans.min(c_10[i + 1] + c_01[n] - c_01[i + 1]);
    }
    println!("{}", ans);
}
