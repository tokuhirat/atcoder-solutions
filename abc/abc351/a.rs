use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        a: [usize; 9],
        b: [usize; 8],
    );
    let a: usize = a.iter().sum();
    let b: usize = b.iter().sum();
    let ans = a - b + 1;
    println!("{}", ans);
}
