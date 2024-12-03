use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        a: [usize; m],
        x: [[usize; m]; n],
    );
    let mut e = vec![0; m];
    for xi in &x {
        for (j, xij) in xi.iter().enumerate() {
            e[j] += xij
        }
    }
    let mut ans = true;
    for (i, &ei) in e.iter().enumerate() {
        if ei < a[i] {
            ans = false;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
