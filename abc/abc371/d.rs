use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        x: [i32; n],
        p: [usize; n],
        q: usize,
    );

    let mut cum_p = Vec::with_capacity(n + 1);
    cum_p.push(0);
    for pi in &p {
        cum_p.push(cum_p.last().unwrap() + pi);
    }

    for _ in 0..q {
        input!(
            (l, r): (i32, i32)
        );
        let l_idx = x.partition_point(|&xi| xi < l);
        let r_idx = x.partition_point(|&xi| xi < r + 1);
        let ans = cum_p[r_idx] - cum_p[l_idx];
        println!("{}", ans);
    }
}
