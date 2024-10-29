use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [[usize; n]; n],
        q: usize,
    );

    let mut row = vec![];
    for i in 0..n {
        row.push(i);
    }

    for _ in 0..q {
        input!(
            t: usize,
            x: usize,
            y: usize,
        );
        if t == 1 {
            (row[x - 1], row[y - 1]) = (row[y - 1], row[x - 1]);
        } else {
            println!("{}", a[row[x - 1]][y - 1]);
        }
    }
}
