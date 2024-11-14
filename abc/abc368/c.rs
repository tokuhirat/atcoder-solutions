use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        h: [usize; n],
    );

    let mut t = 0;
    for hi in &h {
        let p = hi / 5;
        let hi = hi - p * 5;
        let q = match (t + 1) % 3 {
            0 => match hi {
                0 => 0,
                1 => 1,
                2 => 1,
                3 => 1,
                4 => 2,
                _ => unreachable!(),
            },
            1 => match hi {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 3,
                _ => unreachable!(),
            },
            2 => match hi {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 2,
                4 => 2,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        t += 3 * p + q;
    }
    println!("{}", t);
}
