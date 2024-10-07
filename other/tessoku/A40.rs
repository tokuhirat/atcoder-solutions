use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut count = vec![0; *a.iter().max().unwrap() + 1];
    for &ai in a.iter() {
        count[ai] += 1;
    }
    let ans = count
        .iter()
        .fold(0_i128, |acc, e| acc + e * (e - 1) * (e - 2) / 6);
    println!("{}", ans);
}
