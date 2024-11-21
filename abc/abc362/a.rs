use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (r, g, b): (usize, usize, usize),
        c: String,
    );
    if c == "Red" {
        println!("{}", g.min(b));
        return;
    }
    if c == "Green" {
        println!("{}", b.min(r));
        return;
    }
    if c == "Blue" {
        println!("{}", r.min(g));
        return;
    }
}
