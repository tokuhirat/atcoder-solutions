use proconio::{fastout, input};

fn is_ok(a: &[i32]) -> bool {
    let mut cnt = 0;
    for &ai in a {
        if ai > 0 {
            cnt += 1;
        }
    }
    cnt <= 1
}

#[fastout]
fn main() {
    input!(
        n: usize,
        mut a: [i32; n],
    );
    let mut ans = 0;
    while !is_ok(&a) {
        a.sort();
        a[n - 2] -= 1;
        a[n - 1] -= 1;
        ans += 1;
    }
    println!("{}", ans);
}
