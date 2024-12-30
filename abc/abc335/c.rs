use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
    };
    let mut d: VecDeque<_> =
        VecDeque::from((1..=n).map(|e| (e as i32, 0)).collect::<Vec<(i32, i32)>>());
    for _ in 0..q {
        input! {t: u8};
        if t == 1 {
            input! {c: char};
            let now = *d.front().unwrap();
            let mut nxt = now;
            match c {
                'R' => nxt.0 += 1,
                'L' => nxt.0 -= 1,
                'U' => nxt.1 += 1,
                'D' => nxt.1 -= 1,
                _ => unreachable!(),
            }
            d.push_front(nxt);
            d.pop_back();
        } else {
            input! {q: usize};
            println!("{} {}", d[q - 1].0, d[q - 1].1);
        }
    }
}
