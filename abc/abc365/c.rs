use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        mut a: [usize; n],
    );
    a.sort_unstable();
    let mut a_cum = Vec::with_capacity(n + 1);
    a_cum.push(0);
    for &ai in &a {
        a_cum.push(a_cum.last().unwrap() + ai);
    }

    let mut left = 0;
    let mut right = usize::MAX / 2;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        let idx = a.partition_point(|&x| x <= mid);
        if a_cum[idx] + mid * (n - idx) <= m {
            left = mid;
        } else {
            right = mid;
        }
    }
    if right == usize::MAX / 2 {
        println!("infinite");
    } else {
        println!("{}", left);
    }
}
