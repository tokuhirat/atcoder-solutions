use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (_n, k): (usize, usize),
        a: [usize; k],
    };
    let mut ans = 0;
    for i in 0..a.len() / 2 {
        ans += a[2 * i + 1] - a[2 * i];
    }
    if a.len() % 2 != 0 {
        let mut temp = ans;
        for i in (1..a.len()).rev().step_by(2) {
            temp += a[i] - a[i - 1];
            temp -= a[i - 1] - a[i - 2];
            ans = ans.min(temp);
        }
    }
    println!("{}", ans);
}
