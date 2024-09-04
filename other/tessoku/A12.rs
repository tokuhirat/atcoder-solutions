use proconio::input;

fn check(arr: &[u32], v: u32, time: u32) -> bool {
    let mut sum = 0;
    for arr_i in arr {
        sum += time / arr_i;
        if sum >= v {
            return true;
        }
    }
    false
}

fn main() {
    input!(
        n: usize, k: u32,
        a: [u32; n]
    );

    let mut left = 0_u32;
    let mut right = 10_u32.pow(9);

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if check(&a, k, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
