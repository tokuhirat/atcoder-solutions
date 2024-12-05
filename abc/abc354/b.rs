use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut s = Vec::with_capacity(n);
    let mut t = 0;
    for _ in 0..n {
        input!(
            si: String,
            ci: usize,
        );
        s.push(si);
        t += ci;
    }
    s.sort_unstable();
    for (i, si) in s.into_iter().enumerate() {
        if i == t % n {
            println!("{}", si);
            return;
        }
    }
}
