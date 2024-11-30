use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (sx, sy): (i64, i64),
        (tx, ty): (i64, i64),
    );
    let dy = sy.abs_diff(ty) as i64;
    let x_min = sx - dy + if (sx + sy) % 2 == 0 { 0 } else { -1 };
    let x_max = sx + dy + if (sx + sy) % 2 == 0 { 1 } else { 0 };
    if x_min <= tx && tx <= x_max {
        println!("{}", dy);
        return;
    }
    let ans1 = ((tx.abs_diff(x_min) + 1) / 2) as i64;
    let ans2 = ((tx.abs_diff(x_max) + 1) / 2) as i64;
    let ans = dy + ans1.min(ans2);

    println!("{}", ans);
}
