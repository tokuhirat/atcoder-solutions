use proconio::input;
fn main() {
    input!(
        n: i32,
        x: i32,
        a: [i32; n]
    );
    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
