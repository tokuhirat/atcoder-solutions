use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        mut lr: [(usize, usize); n],
    );

    let mut max_l = vec![0; m + 1];

    for &(l, r) in lr.iter() {
        max_l[r] = max_l[r].max(l);
    }

    let mut ans = 0;
    let mut left = 1;

    for right in 1..=m {
        while left <= max_l[right] {
            left += 1;
        }
        ans += right + 1 - left;
    }

    println!("{}", ans);
}
