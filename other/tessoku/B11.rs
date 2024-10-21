use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    );

    a.sort_unstable();

    for &xi in x.iter() {
        let idx = a.partition_point(|&x| x < xi);
        println!("{}", idx);
    }
}
