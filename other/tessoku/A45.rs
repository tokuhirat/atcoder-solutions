use proconio::{input, marker::Chars};

fn main() {
    input!(
        _n: usize,
        c: char,
        a: Chars
    );

    let cnt = a.iter().fold(0, |acc, e| {
        acc + match e {
            'W' => 0,
            'B' => 1,
            'R' => 2,
            _ => 0,
        }
    });

    if (c == 'W' && cnt % 3 == 0) || (c == 'B' && cnt % 3 == 1) || (c == 'R' && cnt % 3 == 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
