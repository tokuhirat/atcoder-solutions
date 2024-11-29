use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [Usize1; 2*n],
    );
    let mut v = vec![vec![]; n];
    for (i, &ai) in a.iter().enumerate() {
        v[ai].push(i);
    }
    let mut ans = 0;
    for vi in &v {
        if vi[1] - vi[0] == 2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
