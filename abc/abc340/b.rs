use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    };
    let mut v = vec![];
    for _ in 0..q {
        input! {(t, x): (usize, usize)};
        if t == 1 {
            v.push(x);
        } else {
            println!("{}", v[v.len() - x]);
        }
    }
}
