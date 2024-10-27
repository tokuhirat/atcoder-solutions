use proconio::{fastout, input};

fn dist_clock(n: usize, start: usize, end: usize) -> usize {
    if end > start {
        end - start
    } else {
        n + end - start
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
        q: usize,
        ht: [(char, usize); q],
    );

    let mut ans = 0;
    let mut left = 1;
    let mut right = 2;

    for &(h, t) in ht.iter() {
        match h {
            'L' => {
                let clock_r = dist_clock(n, left, right);
                let clock_t = dist_clock(n, left, t);
                if clock_r > clock_t {
                    ans += clock_t;
                } else {
                    ans += n - clock_t;
                };
                left = t;
            }
            'R' => {
                let clock_l = dist_clock(n, right, left);
                let clock_t = dist_clock(n, right, t);
                if clock_l > clock_t {
                    ans += clock_t;
                } else {
                    ans += n - clock_t;
                };
                right = t;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
