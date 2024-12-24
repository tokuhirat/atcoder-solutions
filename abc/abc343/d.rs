use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        (n, t): (usize, usize),
    );
    let mut p_to_num = HashMap::new();
    let mut point = vec![0; n];
    p_to_num.insert(0, n);

    for _ in 0..t {
        input!(
            (a, b): (Usize1, usize),
        );
        let e = p_to_num.get_mut(&point[a]).unwrap();
        *e -= 1;
        if *e == 0 {
            p_to_num.remove(&point[a]);
        }
        point[a] += b;
        *p_to_num.entry(point[a]).or_insert(0) += 1;
        println!("{}", p_to_num.len());
    }
}
