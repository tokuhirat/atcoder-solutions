use proconio::{fastout, input, marker::Usize1};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input!(
        (n, q): (i32, usize),
    );
    let mut cnt = vec![1; n as usize];
    let mut set: BTreeMap<_, _> = (0..n).map(|i| (i, i)).collect();
    set.insert(-1, -1);
    set.insert(n, n);

    for _ in 0..q {
        input!(t: u8);
        if t == 1 {
            input!((x, c): (i32, i32));
            let x = x - 1;
            let c = c - 1;
            let (&i, &color) = set.range(..=x).last().unwrap();
            let (&right, &color_r) = set.range(i + 1..).next().unwrap();
            let (_, &color_l) = set.range(..i).last().unwrap();
            cnt[color as usize] -= right - i;
            set.insert(i, c);
            cnt[c as usize] += right - i;
            if color_r == c {
                set.remove(&right);
            }
            if color_l == c {
                set.remove(&i);
            }
        } else {
            input!(c: Usize1);
            println!("{}", cnt[c]);
        }
    }
}
