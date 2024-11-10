use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        q: usize,
    );
    let mut queue = VecDeque::new();
    let mut now = 0;
    let mut ans = vec![];
    for _ in 0..q {
        input!(num: u8);
        match num {
            1 => queue.push_back(now),
            2 => {
                input!(t: usize);
                now += t;
            }
            3 => {
                input!(h: usize);
                let mut cnt: usize = 0;
                while !queue.is_empty() {
                    let &front = queue.front().unwrap();
                    if now - front >= h {
                        queue.pop_front();
                        cnt += 1;
                    } else {
                        break;
                    }
                }
                ans.push(cnt);
            }
            _ => unreachable!(),
        }
    }
    for &ai in &ans {
        println!("{}", ai);
    }
}
