use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        k: usize,
        a: [usize; n],
    );

    let mut a_cumsum = vec![0];
    for &ai in a.iter() {
        a_cumsum.push(a_cumsum.last().unwrap() + ai);
    }

    let mut ans = 0;
    for (i, &ai) in a_cumsum.iter().enumerate() {
        let idx = a_cumsum.binary_search(&(ai + k));
        match idx {
            Err(j) => ans += j - i - 1,
            Ok(j) => ans += j - i,
        };
    }
    println!("{}", ans);
}
