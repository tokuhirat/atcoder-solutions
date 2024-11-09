use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        mut a: [u8; 4],
    );
    a.sort_unstable();
    if a[0] == a[1] {
        if a[2] == a[3] {
            println!("2");
            return;
        } else {
            println!("1");
            return;
        }
    }

    if a[1] == a[2] || a[2] == a[3] {
        println!("1");
        return;
    }
    println!("0");
}
