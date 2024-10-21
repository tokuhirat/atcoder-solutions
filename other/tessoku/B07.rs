use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    );

    let mut count = vec![0; t + 1];
    for &(l, r) in lr.iter() {
        count[l] += 1;
        count[r] += -1;
    }

    let mut num = 0;
    for &ni in count.iter().take(t) {
        num += ni;
        println!("{}", num);
    }
}
