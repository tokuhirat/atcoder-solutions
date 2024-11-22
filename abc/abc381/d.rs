use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut same = vec![];
    for i in 0..n - 1 {
        same.push(a[i] == a[i + 1]);
    }
    let mut right = 0;
    let mut ans = 0;
    let mut seen = HashSet::new();

    let mut left = 0;
    while left < n - 1 {
        if !same[left] {
            left += 2;
            continue;
        }
        seen.insert(a[left]);

        right = right.max(left + 2);
        while right < n - 1 && same[right] && !seen.contains(&a[right]) {
            seen.insert(a[right]);
            right += 2;
        }
        ans = ans.max(right - left);
        seen.remove(&a[left]);
        left += 2;
    }

    left = 1;
    right = 0;
    let mut seen = HashSet::new();
    while left < n - 1 {
        if !same[left] {
            left += 2;
            continue;
        }
        seen.insert(a[left]);

        right = right.max(left + 2);
        while right < n - 1 && same[right] && !seen.contains(&a[right]) {
            seen.insert(a[right]);

            right += 2;
        }
        ans = ans.max(right - left);
        seen.remove(&a[left]);
        left += 2;
    }
    println!("{}", ans);
}
