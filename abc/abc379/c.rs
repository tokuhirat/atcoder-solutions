use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        x: [usize; m],
        a: [usize; m],
    );

    let mut xa: Vec<(_, _)> = x.into_iter().zip(a.into_iter()).collect();
    xa.push((n + 1, 1));
    xa.sort_unstable_by_key(|&(xi, _)| xi);
    let (x, a): (Vec<_>, Vec<_>) = xa.into_iter().unzip();

    let mut ans = 0;
    let mut r = 1;
    let mut x_before = 0;
    for i in 0..=m {
        let l = x[i] - x_before;
        if l > r {
            println!("-1");
            return;
        }
        ans += l * (l - 1) / 2;
        ans += (r - l) * l;

        r -= l;
        r += a[i];
        x_before = x[i];
    }

    if r != 1 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
