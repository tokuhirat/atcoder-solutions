use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );
    let mut s = Vec::with_capacity(n);
    for &ai in &a {
        s.push(ai);

        while s.len() > 1 {
            let b1 = s[s.len() - 1];
            let b2 = s[s.len() - 2];
            if b1 == b2 {
                s.pop();
                s.pop();
                s.push(b1 + 1);
            } else {
                break;
            }
        }
    }
    println!("{}", s.len());
}
