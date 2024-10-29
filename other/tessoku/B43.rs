use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        m: usize,
        a: [usize; m],
    );
    let mut f = vec![0; n];
    for &ai in a.iter() {
        f[ai - 1] += 1;
    }

    for &fi in f.iter() {
        println!("{}", m - fi);
    }
}
