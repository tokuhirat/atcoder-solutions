use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );

    let mut pre_l: usize = 0;
    let mut pre_r: usize = 0;
    let mut ans: usize = 0;
    for _ in 0..n {
        input!(
            (a, s): (usize, char),
        );

        if s == 'L' {
            if pre_l != 0 {
                ans += pre_l.abs_diff(a);
            }
            pre_l = a;
        } else {
            if pre_r != 0 {
                ans += pre_r.abs_diff(a);
            }
            pre_r = a;
        }
    }
    println!("{}", ans);
}
