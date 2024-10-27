use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        c: i32,
        t: [i32; n],
    );

    let mut ans = 0;
    let mut get_time = -c;
    for &ti in t.iter() {
        if ti >= get_time + c {
            ans += 1;
            get_time = ti;
        }
    }
    println!("{}", ans);
}
