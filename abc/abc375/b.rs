use proconio::{fastout, input};

fn dist(xy1: (f64, f64), xy2: (f64, f64)) -> f64 {
    ((xy1.0 - xy2.0).powf(2.0) + (xy1.1 - xy2.1).powf(2.0)).sqrt()
}

#[fastout]
fn main() {
    input!(
        n: usize,
        mut xy: [(f64, f64); n],
    );

    xy.insert(0, (0.0, 0.0));
    xy.insert(n + 1, (0.0, 0.0));

    let mut ans = 0.0;
    for i in 0..n + 1 {
        ans += dist(xy[i], xy[i + 1]);
    }
    println!("{}", ans);
}
