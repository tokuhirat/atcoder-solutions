use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        a: [usize; n],
    );

    let mut right = 0;
    let mut total = 0;
    let mut ans = 0;
    for i in 0..n {
        if i != 0 {
            total -= a[i - 1];
        }
        while right < n && total + a[right] <= k {
            total += a[right];
            right += 1;
        }
        ans += right - i;
    }
    println!("{}", ans);
}
