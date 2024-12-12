use proconio::{fastout, input};

fn solv(ans: &mut Vec<(usize, usize)>, left: usize, right: usize) {
    if left == right {
        return;
    }
    let d = right - left;
    let mut p = d.next_power_of_two();

    loop {
        let j = (left + p - 1) / p;
        let r = p * (j + 1);
        if r <= right {
            ans.push((p * j, p * (j + 1)));
            solv(ans, left, p * j);
            solv(ans, p * (j + 1), right);
            return;
        }
        p >>= 1;
    }
}

#[fastout]
fn main() {
    input!(
        (l, r): (usize, usize),
    );
    let mut ans = vec![];
    solv(&mut ans, l, r);
    ans.sort_unstable();
    println!("{}", ans.len());
    for &a in &ans {
        println!("{} {}", a.0, a.1);
    }
}
