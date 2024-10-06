use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let xor_sum = a.into_iter().reduce(|acc, e| acc ^ e).unwrap();
    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
