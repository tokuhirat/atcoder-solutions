use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut a = vec![];
    loop {
        input!(
            ai: usize,
        );
        a.push(ai);
        if ai == 0 {
            break;
        }
    }
    a.reverse();
    for &ai in &a {
        println!("{}", ai);
    }
}
