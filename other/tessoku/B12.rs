use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut x = vec![0.0_f64];

    while x.last().unwrap() < &47.0 {
        x.push(x.last().unwrap() + 0.0001);
    }
    let idx = x.partition_point(|&x| x.powi(3) + x < n as f64);
    println!("{}", x[idx]);
}
