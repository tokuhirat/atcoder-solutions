use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        q: usize,
        x: [usize; q]
    );

    let size = 300001;
    let mut prime_numbers = vec![true; size];
    for i in 0..size {
        if i < 2 || !prime_numbers[i] {
            continue;
        }
        let mut j = 2 * i;
        while j < size {
            prime_numbers[j] = false;
            j += i;
        }
    }

    for &xi in x.iter() {
        if prime_numbers[xi] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
