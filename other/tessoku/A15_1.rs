use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i32; n]
    );

    let mut b = a.clone();
    b.sort_unstable();
    b.dedup();

    for &ai in a.iter() {
        let idx = b.partition_point(|&x| x < ai);
        print!("{} ", idx + 1);
    }
    println!(" ");
}
