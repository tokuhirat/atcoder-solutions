use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );
    let mut b = a.clone();
    b.sort_unstable();
    let m = b[b.len() - 2];
    for (i, &ai) in a.iter().enumerate() {
        if ai == m {
            println!("{}", i + 1);
            return;
        }
    }
}
