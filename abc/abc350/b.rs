use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        t: [Usize1; q],
    );
    let mut b = vec![true; n];
    let mut ans = n;
    for &ti in &t {
        if b[ti] {
            ans -= 1;
        } else {
            ans += 1;
        }
        b[ti] = !b[ti];
    }
    println!("{}", ans);
}
