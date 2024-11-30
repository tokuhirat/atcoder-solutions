use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
        a: [usize; n],
        b: [usize; m],
    );

    let mut aa: Vec<(usize, usize)> = Vec::with_capacity(n);
    for (i, &ai) in a.iter().enumerate() {
        if i == 0 || ai < aa.last().unwrap().0 {
            aa.push((ai, i + 1));
        }
    }
    let mut ans = Vec::with_capacity(m);
    aa.reverse();
    for bi in &b {
        let i = aa.binary_search_by(|&e| e.0.cmp(bi));
        match i {
            Ok(i) => {
                ans.push(aa[i].1 as i64);
            }
            Err(i) => {
                if i == 0 {
                    ans.push(-1);
                } else {
                    ans.push(aa[i - 1].1 as i64);
                }
            }
        }
    }
    for ans_i in &ans {
        println!("{}", ans_i);
    }
}
