use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (mut a, mut b, c): (u32, u32, usize),
    );
    let n = 60;
    let diff_bit = c.count_ones();
    let mut one_bit = a + b - diff_bit;
    if one_bit % 2 == 1 {
        println!("-1");
        return;
    }
    one_bit /= 2;
    if a < one_bit || b < one_bit {
        println!("-1");
        return;
    }
    a -= one_bit;
    b -= one_bit;
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        if c & (1 << i) != 0 {
            if a > 0 {
                x[i] = 1;
                a -= 1;
            } else if b > 0 {
                y[i] = 1;
                b -= 1;
            }
        } else if one_bit > 0 {
            x[i] = 1;
            y[i] = 1;
            one_bit -= 1;
        }
    }
    if a > 0 || b > 0 || one_bit > 0 {
        println!("-1");
        return;
    }
    let x = x
        .iter()
        .enumerate()
        .fold(0_usize, |acc, (i, &e)| acc + (1 << i) * e);
    let y = y
        .iter()
        .enumerate()
        .fold(0_usize, |acc, (i, &e)| acc + (1 << i) * e);
    println!("{} {}", x, y);
}
