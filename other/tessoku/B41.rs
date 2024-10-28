use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut x: usize,
        mut y: usize,
    );

    let mut ans = vec![];
    while x > 1 || y > 1 {
        ans.push((x, y));
        if x > y {
            x -= y;
        } else {
            y -= x
        }
    }
    println!("{}", ans.len());
    for &(xi, yi) in ans.iter().rev() {
        println!("{} {}", xi, yi);
    }
}
