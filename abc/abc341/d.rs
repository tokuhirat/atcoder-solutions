use proconio::{fastout, input};

fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a;
    }
    gcd(a % b, b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn f(x: usize, n: usize, m: usize, l: usize, k: usize) -> bool {
    let cnt = x / n + x / m - x / l * 2;
    cnt < k
}

#[fastout]
fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
    };
    let l = lcm(n, m);
    let mut left = 0;
    let mut right = usize::MAX;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if f(mid, n, m, l, k) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", right);
}
