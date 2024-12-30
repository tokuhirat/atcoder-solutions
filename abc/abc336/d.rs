use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut l = Vec::with_capacity(n);
    for (i, &ai) in a.iter().enumerate() {
        if i == 0 {
            l.push(1);
        } else {
            l.push(ai.min(l.last().unwrap() + 1));
        }
    }

    let mut r = Vec::with_capacity(n);
    for (i, &ai) in a.iter().rev().enumerate() {
        if i == 0 {
            r.push(1);
        } else {
            r.push(ai.min(r.last().unwrap() + 1));
        }
    }
    r.reverse();
    let ans = l
        .iter()
        .zip(r.iter())
        .map(|(li, ri)| li.min(ri))
        .max()
        .unwrap();
    println!("{}", ans);
}
