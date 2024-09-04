use proconio::input;
fn main() {
    input!(
        n: usize,
        a: [u32; n],
        d: usize,
        lr: [(usize, usize); d]
    );

    let mut max_l = vec![0; n];
    max_l[0] = a[0];
    for i in 1..n {
        max_l[i] = max_l[i - 1].max(a[i]);
    }
    let mut max_r = vec![0; n];
    max_r[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        max_r[i] = max_r[i + 1].max(a[i]);
    }

    for (l, r) in lr {
        let ans = max_l[l - 2].max(max_r[r]);
        println!("{}", ans);
    }
}
