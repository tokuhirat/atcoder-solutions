use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        (n, m): (usize, usize),
    );

    let mut born = vec![false; n];
    for _ in 0..m {
        input!(
            (a, b): (Usize1, char)
        );
        if !born[a] && b == 'M' {
            born[a] = true;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
