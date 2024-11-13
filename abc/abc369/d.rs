use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [i64; n],
    );

    let mut odd = vec![];
    let mut even = vec![];
    for (i, &ai) in a.iter().enumerate() {
        if i == 0 {
            odd.push(ai);
            even.push(0);
        } else {
            let &e = even.last().unwrap();
            let &o = odd.last().unwrap();
            odd.push(o.max(e + ai));
            even.push(e.max(o + 2 * ai));
        }
    }
    let ans = odd.last().unwrap().max(even.last().unwrap());
    println!("{}", ans);
}
