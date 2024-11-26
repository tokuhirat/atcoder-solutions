use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input!(
        (n, t): (usize, i64),
        s: Chars,
        x: [i64; n],
    );
    let mut left = Vec::with_capacity(n);
    let mut right = Vec::with_capacity(n);
    for (i, &si) in s.iter().enumerate() {
        match si {
            '0' => {
                left.push(x[i]);
            }
            '1' => {
                right.push(x[i]);
            }
            _ => unreachable!(),
        }
    }
    left.sort_unstable();
    let mut ans = 0;
    for &r in &right {
        let li = left.partition_point(|&e| e < r);
        let ri = left.partition_point(|&e| e <= r + 2 * t);
        ans += ri - li;
    }
    println!("{}", ans);
}
