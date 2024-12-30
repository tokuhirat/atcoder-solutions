use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut t = 0;
    let mut a = 0;
    for _ in 0..n {
        input! {(x, y): (usize, usize)};
        t += x;
        a += y;
    }
    match t.cmp(&a) {
        std::cmp::Ordering::Equal => {
            println!("Draw");
        }
        std::cmp::Ordering::Greater => {
            println!("Takahashi");
        }
        std::cmp::Ordering::Less => {
            println!("Aoki");
        }
    }
}
