use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input!(
        q: usize,
    );

    let mut map = HashMap::new();
    for _ in 0..q {
        input!(ty: u8);
        if ty == 1 {
            input!(x: usize);
            let e = map.entry(x).or_insert(0);
            *e += 1;
        } else if ty == 2 {
            input!(x: usize);
            let e = map.get_mut(&x).unwrap();
            *e -= 1;
            if *e == 0 {
                map.remove(&x);
            }
        } else {
            println!("{}", map.len());
        }
    }
}
