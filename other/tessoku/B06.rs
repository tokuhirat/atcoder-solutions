use std::cmp::Ordering;

use proconio::input;

fn main() {
    input!(
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    );

    let mut a_cum = vec![0; n + 1];
    for i in 0..n {
        a_cum[i + 1] = a_cum[i] + a[i];
    }

    for &(l, r) in lr.iter() {
        let num = r - l + 1;
        let win = a_cum[r] - a_cum[l - 1];

        match num.cmp(&(2 * win)) {
            Ordering::Less => println!("win"),
            Ordering::Greater => println!("lose"),
            Ordering::Equal => println!("draw"),
        }
    }
}
