use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, k): (usize, usize),
        mut a: [usize; n],
    );
    a.sort_unstable();
    let mut ans = usize::MAX;

    for left in 0..=k {
        let right = left + n - k - 1;
        ans = ans.min(a[right] - a[left]);
    }

    println!("{}", ans);
}
