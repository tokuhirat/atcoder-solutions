use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        (n, q): (usize, usize),
        mut a: [i32; n],
    );
    a.sort_unstable();
    for _ in 0..q {
        input!(
            (b, k): (i32, usize)
        );
        let mut near = -1;
        let mut far = 200_000_000;
        while far - near > 1 {
            let mid = near + (far - near) / 2;
            let left = b - mid;
            let right = b + mid;
            let li = a.partition_point(|&x| x < left);
            let ri = a.partition_point(|&x| x < right + 1);
            if ri - li < k {
                near = mid;
            } else {
                far = mid;
            }
        }
        println!("{}", far);
    }
}
