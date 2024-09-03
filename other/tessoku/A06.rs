use proconio::input;
fn main() {
    input!(n: u32, q: u32);
    input!(a: [u32; n]);
    input!(lr: [[usize; 2]; q]);

    let mut a_sum = vec![0];
    for (i, ai) in a.iter().enumerate() {
        a_sum.push(a_sum[i] + *ai);
    }
    for lri in lr {
        let ans = a_sum[lri[1]] - a_sum[lri[0] - 1];
        println!("{}", ans);
    }
}
