use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        mut r: [usize; n],
    };
    r.sort_unstable();
    let r_cum: Vec<_> = r
        .iter()
        .scan(0, |acc, &e| {
            *acc += e;
            Some(*acc)
        })
        .collect();
    for _ in 0..q {
        input! {x: usize};
        let ans = r_cum.partition_point(|&e| e <= x);
        println!("{}", ans);
    }
}
