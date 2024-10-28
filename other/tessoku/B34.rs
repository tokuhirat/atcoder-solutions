use proconio::{fastout, input};

fn calc_g(num: usize) -> usize {
    match (num + 1) % 5 {
        0 => 2,
        1 => 0,
        2 => 0,
        3 => 1,
        4 => 1,
        _ => unreachable!(),
    }
}

#[fastout]
fn main() {
    input!(
        n: usize,
        _x: usize,  //2
        _y: usize,  //3
        a: [usize; n]
    );

    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    // 0, 0, 1, 1, 2, 0, 0, 1, 1, 2
    let mut xor_sum = 0;
    for &ai in a.iter() {
        xor_sum ^= calc_g(ai);
    }

    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
