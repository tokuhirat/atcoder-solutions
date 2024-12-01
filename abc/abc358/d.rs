use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        mut a: [usize; n],
        mut b: [usize; m],
    );
    a.sort_unstable();
    b.sort_unstable();
    let mut ans = 0;
    let mut i = 0;
    for &bi in &b {
        while i < n && a[i] < bi {
            i += 1;
        }
        if i == n {
            println!("-1");
            return;
        }
        ans += a[i];
        i += 1;
    }
    println!("{}", ans);
}
