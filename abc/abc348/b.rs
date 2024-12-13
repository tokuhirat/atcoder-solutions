use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        xy: [(i32, i32); n],
    );
    for (i, xyi) in xy.iter().enumerate() {
        let mut max = i32::MIN;
        let mut idx = 0;
        for (j, xyj) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            let d = (xyi.0 - xyj.0) * (xyi.0 - xyj.0) + (xyi.1 - xyj.1) * (xyi.1 - xyj.1);
            if d > max {
                max = d;
                idx = j
            }
        }
        println!("{}", idx + 1);
    }
}
