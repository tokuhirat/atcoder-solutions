#![allow(clippy::needless_range_loop)]
use proconio::{fastout, input};
use std::mem::swap;

fn f(x: usize) -> usize {
    let x: Vec<char> = x.to_string().chars().collect();
    let mut dp = vec![vec![vec![0; 2]; 2]; 10];
    dp[0][1][1] = 1;
    let x0 = (x[0] as u8 - b'0') as usize;
    for i in 1..x0 {
        dp[i][1][0] = 1;
    }
    dp[x0][0][0] = 1;
    for i in 1..x.len() {
        let mut old = vec![vec![vec![0; 2]; 2]; 10];
        swap(&mut dp, &mut old);
        for j in 0..10 {
            for is_under in 0..2 {
                for is_zero in 0..2 {
                    for k in 0..10_usize {
                        if is_zero == 0 && k >= j {
                            continue;
                        }
                        if is_under == 0 && k as u8 > x[i] as u8 - b'0' {
                            continue;
                        }
                        let nj = if is_zero == 1 && k != 0 { k } else { j };
                        let n_is_under = if is_under == 1 || (k as u8) < x[i] as u8 - b'0' {
                            1
                        } else {
                            0
                        };
                        let n_is_zero = if is_zero == 0 || k > 0 { 0 } else { 1 };
                        dp[nj][n_is_under][n_is_zero] += old[j][is_under][is_zero];
                    }
                }
            }
        }
    }
    let mut ret = 0;
    for i in 0..10 {
        for j in 0..2 {
            ret += dp[i][j][0];
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    };

    let ans = f(r) - f(l - 1);
    println!("{}", ans);
}
