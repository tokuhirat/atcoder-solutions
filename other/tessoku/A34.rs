use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    );

    let size = *a.iter().max().unwrap();
    let mut grundy = vec![0; size + 1];
    for i in 0..=size {
        let mut transition = [false, false, false];
        if i >= x {
            transition[grundy[i - x]] = true;
        }
        if i >= y {
            transition[grundy[i - y]] = true;
        }
        if !transition[0] {
            grundy[i] = 0;
        } else if !transition[1] {
            grundy[i] = 1;
        } else {
            grundy[i] = 2;
        }
    }

    let xor_sum = a.iter().fold(0, |acc, &e| acc ^ grundy[e]);
    if xor_sum > 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
