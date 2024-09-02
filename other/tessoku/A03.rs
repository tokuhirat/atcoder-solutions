use proconio::input;
fn main() {
    input!(
        n: i32,
        k: i32,
        p: [i32; n],
        q: [i32; n]
    );
    for pi in &p {
        if q.contains(&(k - pi)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
