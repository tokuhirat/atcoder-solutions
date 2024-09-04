use proconio::input;
fn main() {
    input!(
        n: usize, x: u32,
        a: [u32; n]
    );

    let mut left: i32 = -1;
    let mut right: i32 = n as i32;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if a[mid as usize] >= x {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right + 1);
}
