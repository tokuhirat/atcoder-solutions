use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        a: [usize; n],
        b: [usize; m],
    );
    let mut x = [a.clone(), b].concat();
    x.sort_unstable();
    for i in 0..n + m - 1 {
        if a.contains(&x[i]) && a.contains(&x[i + 1]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
