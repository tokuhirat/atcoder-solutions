use proconio::{fastout, input};

const SHIFT: i64 = 1000000000;

#[fastout]
fn main() {
    input!(
        (a, b, c, d): (i64, i64, i64, i64),
    );
    let a = a + SHIFT;
    let b = b + SHIFT;
    let c = c + SHIFT;
    let d = d + SHIFT;

    let x = c - a;
    let y = d - b;
    let y_num = y / 2;

    let mut v = [0; 4];
    v[0] = 3 * y_num;
    if y % 2 == 1 {
        if d % 2 == 0 {
            v[0] += 1;
        } else {
            v[0] += 2;
        }
    }

    v[1] = 3 * y_num;
    if y % 2 == 1 {
        if d % 2 == 0 {
            v[1] += 2;
        } else {
            v[1] += 1;
        }
    }

    v[2] = y_num;
    if y % 2 == 1 && d % 2 == 0 {
        v[2] += 1;
    }

    v[3] = y_num;
    if y % 2 == 1 && d % 2 == 1 {
        v[3] += 1;
    }
    let mut ans: i64 = v.iter().sum::<i64>() * (x / 4);
    for xi in 0..x % 4 {
        ans += v[((xi + a) % 4) as usize];
    }

    println!("{}", ans);
}
