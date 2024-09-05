use proconio::input;

fn main() {
    input!(
        n: usize, k: u32,
        a: [u32; n]
    );

    let mut ans = 0;
    for i in 0..n {
        let mut left = i;
        let mut right = n;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if a[mid] - a[i] <= k {
                left = mid;
            } else {
                right = mid;
            }
        }
        ans += left - i;
    }
    println!("{}", ans);
}
