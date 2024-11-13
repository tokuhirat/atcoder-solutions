use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i32; n],
    );

    let mut ans = 0;
    let mut left = 0;

    while left < n - 1 {
        let mut right = left + 1;
        let diff = a[right] - a[left];
        while right < n - 1 && a[right + 1] - a[right] == diff {
            right += 1;
        }
        ans += (right - left + 1) * (right - left) / 2;
        left = right;
    }
    ans += n;
    println!("{}", ans);
}
