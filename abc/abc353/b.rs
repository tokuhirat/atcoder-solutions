use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        a: [usize; n],
    );
    let mut ans = 0;
    let mut now = 0;
    for &ai in &a {
        if ai + now <= k {
            now += ai;
        } else {
            ans += 1;
            now = ai;
        }
    }
    if now > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
