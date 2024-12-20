use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (mut w, mut b): (usize, usize),
    );
    if w + b > 12 {
        let n = (w + b) / 12;
        if w < 7 * n || b < 5 * n {
            println!("No");
            return;
        }
        w -= 7 * n;
        b -= 5 * n;
    }
    let s: Vec<char> = "wbwbwwbwbwbwwbwbwwbwbwbw".chars().collect();
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            let ss = &s[i..j];
            if ss.iter().filter(|&e| *e == 'w').count() == w
                && ss.iter().filter(|&e| *e == 'b').count() == b
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
