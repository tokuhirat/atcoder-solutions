use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
    );
    let mut l = Vec::with_capacity(n);
    let mut r = Vec::with_capacity(n);
    let mut l_sum = 0;
    let mut r_sum = 0;
    for _ in 0..n {
        input!((li, ri): (isize, isize));
        l.push(li);
        r.push(ri);
        l_sum += li;
        r_sum += ri;
    }
    if l_sum > 0 || r_sum < 0 {
        println!("No");
        return;
    }
    println!("Yes");
    let mut res = r_sum;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let mut ri = r[i];
        while res > 0 && ri > l[i] {
            ri -= 1;
            res -= 1;
        }
        ans.push(ri);
    }
    println!("{}", ans.iter().join(" "));
}
