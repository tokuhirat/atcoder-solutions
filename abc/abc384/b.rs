use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, mut r): (usize, i32),
    );
    for _ in 0..n {
        input!((d, a): (usize, i32));
        match d {
            1 => {
                if (1600..=2799).contains(&r) {
                    r += a;
                }
            }
            2 => {
                if (1200..=2399).contains(&r) {
                    r += a;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", r);
}
